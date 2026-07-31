## Consortium ORT: ONNX Runtime Integration

The Consortium ORT (ONNX Runtime Wrapper) provides seamless AI model inference across all components (M-core, A-core, TEE) with automatic model bundling, hardware acceleration selection, and type-safe inference.

### ORT Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Consortium Application                                     │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Inference Request                                   │  │
│  │  let output = model.predict(&input).await?           │  │
│  └────────┬────────────────────────────────────────────┘  │
│           │                                                 │
│  ┌────────▼────────────────────────────────────────────┐  │
│  │  Consortium ORT Layer                               │  │
│  │  ├─ Model loader (ONNX .onnx file)                 │  │
│  │  ├─ Provider selector (CPU, GPU, NPU, etc.)        │  │
│  │  ├─ Session manager (pooling, caching)             │  │
│  │  └─ Type-safe I/O (ndarray, glam, etc.)            │  │
│  └────────┬────────────────────────────────────────────┘  │
│           │                                                 │
│  ┌────────▼────────────────────────────────────────────┐  │
│  │  ONNX Runtime (ort crate)                           │  │
│  │  ├─ Model parsing (protobuf)                        │  │
│  │  ├─ Graph execution (CPU threading)                 │  │
│  │  └─ Memory management (arena allocators)            │  │
│  └────────┬────────────────────────────────────────────┘  │
│           │                                                 │
│  ┌────────▼────────────────────────────────────────────┐  │
│  │  Hardware Providers (Optional)                      │  │
│  │  ├─ NVIDIA CUDA (GPU inference)                     │  │
│  │  ├─ OpenVINO (Intel NPU/Movidius)                   │  │
│  │  ├─ QNN (Qualcomm Hexagon DSP)                      │  │
│  │  └─ CoreML (Apple Neural Engine)                    │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Developer Experience: Model-First API

#### Step 1: Define Inference Schema

```rust
// my-app/src/models.rs
use consortium_ort::Model;
use ndarray::Array2;

// 1. Define input/output types
pub struct ObjectDetectionInput {
    pub image: Array3<f32>,  // [height, width, 3] (RGB)
}

pub struct ObjectDetectionOutput {
    pub boxes: Vec<[f32; 4]>,      // [x1, y1, x2, y2]
    pub confidences: Vec<f32>,     // [0.0, 1.0]
    pub class_ids: Vec<u32>,       // class index
}

// 2. Define model type
pub struct ObjectDetectionModel {
    session: ort::Session,
}

impl Model for ObjectDetectionModel {
    type Input = ObjectDetectionInput;
    type Output = ObjectDetectionOutput;

    async fn load(path: &str) -> Result<Self> {
        let session = ort::Session::builder()
            .with_model_from_file(path)?
            .build()?;
        Ok(Self { session })
    }

    async fn predict(&self, input: &Self::Input) -> Result<Self::Output> {
        // 1. Prepare input tensor
        let input_tensor = input.image.view();

        // 2. Run inference
        let outputs = self.session.run(
            ort::inputs![input_tensor.into()]?
        )?;

        // 3. Parse output tensors
        let boxes_raw = outputs[0].try_extract_tensor::<f32>()?;
        let confs_raw = outputs[1].try_extract_tensor::<f32>()?;
        let classes_raw = outputs[2].try_extract_tensor::<i32>()?;

        // 4. Convert to user types
        let boxes = boxes_raw
            .iter()
            .chunks(4)
            .into_iter()
            .map(|mut chunk| [
                chunk.next().unwrap().clone(),
                chunk.next().unwrap().clone(),
                chunk.next().unwrap().clone(),
                chunk.next().unwrap().clone(),
            ])
            .collect();

        let confidences = confs_raw.to_vec();
        let class_ids = classes_raw
            .iter()
            .map(|&id| id as u32)
            .collect();

        Ok(ObjectDetectionOutput {
            boxes,
            confidences,
            class_ids,
        })
    }
}
```

#### Step 2: Load & Predict with Full Workflow

```rust
// my-app/src/models/detection.rs
use ndarray::Array3;
use std::sync::Arc;
use std::path::Path;

/// Load and preprocess image
pub fn load_and_preprocess(path: &Path) -> Result<ObjectDetectionInput> {
    use image::io::Reader as ImageReader;

    // 1. Load image
    let img = ImageReader::open(path)?
        .decode()?
        .to_rgb8();

    // 2. Resize to model input size (e.g., 640x640)
    let resized = image::imageops::resize(&img, 640, 640, image::imageops::FilterType::Lanczos3);

    // 3. Convert to ndarray and normalize
    let arr = ndarray::Array3::from_shape_fn((640, 640, 3), |(y, x, c)| {
        resized.get_pixel(x as u32, y as u32)[c] as f32 / 255.0
    });

    Ok(ObjectDetectionInput { image: arr })
}

/// Post-process model outputs
pub fn postprocess(output: &ObjectDetectionOutput, conf_threshold: f32) -> Vec<Detection> {
    let mut detections = Vec::new();

    for (i, conf) in output.confidences.iter().enumerate() {
        if *conf >= conf_threshold {
            detections.push(Detection {
                bbox: output.boxes[i],
                confidence: *conf,
                class_id: output.class_ids[i],
                class_name: get_class_name(output.class_ids[i]),
            });
        }
    }

    // Sort by confidence descending
    detections.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

    detections
}

#[derive(Debug, Clone)]
pub struct Detection {
    pub bbox: [f32; 4],
    pub confidence: f32,
    pub class_id: u32,
    pub class_name: String,
}

fn get_class_name(id: u32) -> String {
    match id {
        0 => "person".to_string(),
        1 => "bicycle".to_string(),
        2 => "car".to_string(),
        // ... etc
        _ => format!("class_{}", id),
    }
}
```

#### Step 3: Complete Application with Model Management

```rust
// my-app/src/main.rs
use my_app::models::{
    ObjectDetectionModel,
    ObjectDetectionInput,
    load_and_preprocess,
    postprocess,
};
use consortium_ort::ModelPool;
use std::time::Instant;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // 1. Load model from bundled assets
    tracing::info!("Loading YOLO model...");
    let model = ObjectDetectionModel::load(
        "/usr/share/consortium/models/yolov8n.onnx"
    ).await?;

    // 2. Create inference session pool (one per CPU core)
    let pool_size = num_cpus::get();
    tracing::info!("Creating session pool (size: {})", pool_size);
    let pool = ModelPool::new(model, pool_size)?;

    // 3. Batch inference on multiple images
    let image_paths = vec![
        "test1.jpg",
        "test2.jpg",
        "test3.jpg",
    ];

    tracing::info!("Processing {} images", image_paths.len());
    let batch_start = Instant::now();

    let mut tasks = Vec::new();
    for path in &image_paths {
        let pool_clone = pool.clone();
        let path = path.to_string();

        tasks.push(tokio::spawn(async move {
            process_image(&pool_clone, &path).await
        }));
    }

    // Await all tasks
    let results = futures::future::join_all(tasks).await;

    // 4. Aggregate results
    for (path, result) in image_paths.iter().zip(results.iter()) {
        match result {
            Ok(Ok(detections)) => {
                tracing::info!("✓ {}: {} objects detected", path, detections.len());
                for det in detections.iter().take(3) {
                    tracing::info!(
                        "  - {} ({:.2}): {:?}",
                        det.class_name, det.confidence, det.bbox
                    );
                }
            }
            Ok(Err(e)) => {
                tracing::error!("✗ {}: {}", path, e);
            }
            Err(e) => {
                tracing::error!("✗ {}: Task error: {}", path, e);
            }
        }
    }

    let elapsed = batch_start.elapsed();
    tracing::info!("Batch processing completed in {:.2}s", elapsed.as_secs_f32());

    // 5. Real-time stream example (from video or camera)
    process_video_stream(&pool).await?;

    Ok(())
}

/// Process a single image
async fn process_image(
    pool: &ModelPool<ObjectDetectionModel>,
    image_path: &str,
) -> Result<Vec<Detection>> {
    let start = Instant::now();

    // 1. Load and preprocess
    let input = load_and_preprocess(Path::new(image_path))?;

    // 2. Run inference
    let output = pool.predict(&input).await?;

    // 3. Post-process with confidence threshold
    let detections = postprocess(&output, 0.5);

    let elapsed = start.elapsed();
    tracing::debug!("{}: inference took {:.2}ms", image_path, elapsed.as_secs_f32() * 1000.0);

    Ok(detections)
}

/// Process video stream (e.g., from USB camera or RTSP stream)
async fn process_video_stream(
    pool: &ModelPool<ObjectDetectionModel>,
) -> Result<()> {
    use opencv::videoio::VideoCapture;
    use opencv::core::Mat;

    tracing::info!("Starting video stream processing...");

    let mut cap = VideoCapture::new_default(0)?; // Camera index 0
    let mut frame = Mat::default();

    let mut frame_count = 0;
    let stream_start = Instant::now();

    loop {
        if !cap.read(&mut frame)? {
            break;
        }

        frame_count += 1;

        // Skip frames for real-time performance (process every 3rd frame)
        if frame_count % 3 != 0 {
            continue;
        }

        // Convert OpenCV Mat to ndarray
        // (simplified; actual implementation depends on opencv crate version)
        let height = frame.rows() as usize;
        let width = frame.cols() as usize;

        // Prepare input
        let mut input_data = Vec::with_capacity(height * width * 3);
        for pixel in frame.data_mut().iter() {
            input_data.push(*pixel as f32 / 255.0);
        }

        let input = ObjectDetectionInput {
            image: ndarray::Array3::from_shape_vec(
                (height, width, 3),
                input_data,
            )?,
        };

        // Run inference
        match pool.predict(&input).await {
            Ok(output) => {
                let detections = postprocess(&output, 0.5);
                if !detections.is_empty() {
                    tracing::info!("Frame {}: {} objects", frame_count, detections.len());
                }
            }
            Err(e) => {
                tracing::error!("Inference error: {}", e);
            }
        }

        // Print stats every 100 frames
        if frame_count % 100 == 0 {
            let elapsed = stream_start.elapsed().as_secs_f32();
            let fps = frame_count as f32 / elapsed;
            tracing::info!("Processing: {:.1} FPS", fps);
        }
    }

    let total_elapsed = stream_start.elapsed();
    tracing::info!(
        "Video stream complete: {} frames in {:.2}s ({:.1} avg FPS)",
        frame_count,
        total_elapsed.as_secs_f32(),
        frame_count as f32 / total_elapsed.as_secs_f32()
    );

    Ok(())
}
```

#### Step 4: Model Performance Monitoring

```rust
// my-app/src/models/monitoring.rs
use std::time::Instant;
use std::sync::{Arc, Mutex};

pub struct InferenceMetrics {
    pub total_inferences: u64,
    pub total_latency_ms: f64,
    pub min_latency_ms: f64,
    pub max_latency_ms: f64,
    pub errors: u64,
}

impl InferenceMetrics {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            total_inferences: 0,
            total_latency_ms: 0.0,
            min_latency_ms: f64::INFINITY,
            max_latency_ms: 0.0,
            errors: 0,
        }))
    }

    pub fn record(&mut self, latency_ms: f64) {
        self.total_inferences += 1;
        self.total_latency_ms += latency_ms;
        self.min_latency_ms = self.min_latency_ms.min(latency_ms);
        self.max_latency_ms = self.max_latency_ms.max(latency_ms);
    }

    pub fn record_error(&mut self) {
        self.errors += 1;
    }

    pub fn average_latency_ms(&self) -> f64 {
        if self.total_inferences == 0 {
            0.0
        } else {
            self.total_latency_ms / self.total_inferences as f64
        }
    }

    pub fn print_summary(&self) {
        tracing::info!("=== Inference Metrics ===");
        tracing::info!("Total inferences: {}", self.total_inferences);
        tracing::info!("Errors: {}", self.errors);
        tracing::info!("Average latency: {:.2}ms", self.average_latency_ms());
        tracing::info!("Min latency: {:.2}ms", self.min_latency_ms);
        tracing::info!("Max latency: {:.2}ms", self.max_latency_ms);
        if self.total_inferences > 0 {
            let throughput = 1000.0 / self.average_latency_ms();
            tracing::info!("Throughput: {:.1} inferences/sec", throughput);
        }
    }
}

// Usage in inference loop
pub async fn monitored_inference(
    pool: &ModelPool<ObjectDetectionModel>,
    input: &ObjectDetectionInput,
    metrics: Arc<Mutex<InferenceMetrics>>,
) -> Result<ObjectDetectionOutput> {
    let start = Instant::now();

    match pool.predict(input).await {
        Ok(output) => {
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            metrics.lock().unwrap().record(elapsed);
            Ok(output)
        }
        Err(e) => {
            metrics.lock().unwrap().record_error();
            Err(e)
        }
    }
}
```

### ORT Model Quantization & Bundling

The build system provides a complete pipeline for model optimization and bundling:

#### Step 1: Define Models in Cargo.toml

```toml
# my-app/Cargo.toml
[package]
name = "my-app"

# Models to include
[package.metadata.consortium-models]
yolov8n = { path = "models/yolov8n.onnx", quantize = "int8", providers = ["cpu"] }
resnet50 = { path = "models/resnet50.onnx", quantize = "fp16", providers = ["cpu", "gpu"] }
pose = { path = "models/pose.onnx", quantize = false, providers = ["cpu"] }

[build-dependencies]
consortium-builder = { path = "../../consortium-builder" }
```

#### Step 2: Build Script with Quantization

```rust
// build.rs
use consortium_builder::{ModelOptimizer, ModelBundle};

fn main() {
    let models = vec![
        ModelConfig {
            name: "yolov8n",
            path: "models/yolov8n.onnx",
            quantization: Quantization::Int8,
            hardware_providers: vec!["cpu"],
            calibration_dataset: Some("data/calibration_images/"),
        },
        ModelConfig {
            name: "resnet50",
            path: "models/resnet50.onnx",
            quantization: Quantization::FP16,
            hardware_providers: vec!["cpu", "gpu"],
            calibration_dataset: None,  // FP16 doesn't need calibration
        },
        ModelConfig {
            name: "pose",
            path: "models/pose.onnx",
            quantization: Quantization::None,
            hardware_providers: vec!["cpu"],
            calibration_dataset: None,
        },
    ];

    for model in models {
        println!("Processing model: {}", model.name);

        // 1. Load original ONNX
        let mut onnx_model = ort::Model::load(&model.path)?;

        // 2. Quantize (if specified)
        let quantized_model = match model.quantization {
            Quantization::Int8 => {
                println!("  → Quantizing to INT8...");
                let calibrator = ModelOptimizer::create_calibrator(
                    &onnx_model,
                    model.calibration_dataset.unwrap(),
                )?;
                ort::quantization::quantize_dynamic(
                    &onnx_model,
                    Some(calibrator),
                    ort::quantization::QuantType::INT8,
                )?
            }
            Quantization::FP16 => {
                println!("  → Converting to FP16...");
                ort::quantization::convert_float_to_float16(&onnx_model)?
            }
            Quantization::None => onnx_model,
        };

        // 3. Optimize graph (operator fusion, constant folding, etc.)
        println!("  → Optimizing graph...");
        let optimized_model = ModelOptimizer::optimize(
            &quantized_model,
            &model.hardware_providers,
        )?;

        // 4. Serialize to binary format with metadata
        let out_dir = env::var("OUT_DIR").unwrap();
        let model_path = format!("{}/models/{}.ort", out_dir, model.name);

        let bundle = ModelBundle {
            onnx_bytes: optimized_model.to_bytes()?,
            input_shapes: optimized_model.input_shapes().clone(),
            input_types: optimized_model.input_types().clone(),
            output_shapes: optimized_model.output_shapes().clone(),
            output_types: optimized_model.output_types().clone(),
            quantization: model.quantization,
            hardware_providers: model.hardware_providers,
            size_original: std::fs::metadata(&model.path)?.len(),
            size_quantized: optimized_model.to_bytes()?.len(),
        };

        bundle.write_to_file(&model_path)?;

        println!(
            "  ✓ {} → {} ({:.1}% size reduction)",
            model.name,
            model_path,
            (1.0 - (bundle.size_quantized as f64 / bundle.size_original as f64)) * 100.0
        );
    }
}
```

#### Step 3: Load Models with Metadata

```rust
// my-app/src/models.rs
use consortium_ort::{ModelBundle, Model};

#[tokio::main]
async fn main() -> Result<()> {
    // Models are pre-compiled and bundled at build time
    // Read from embedded binary or file system
    let yolo_bundle = ModelBundle::load_embedded("yolov8n")?;
    let resnet_bundle = ModelBundle::load_embedded("resnet50")?;

    println!("YOLOv8n:");
    println!("  Quantization: {:?}", yolo_bundle.quantization);
    println!("  Original size: {} MB", yolo_bundle.size_original / 1_000_000);
    println!("  Quantized size: {} MB", yolo_bundle.size_quantized / 1_000_000);
    println!("  Compression: {:.1}%",
        (1.0 - yolo_bundle.size_quantized as f64 / yolo_bundle.size_original as f64) * 100.0);

    // Create session with auto-selected hardware provider
    let session = yolo_bundle.create_session()?;

    Ok(())
}
```

### Quantization Impact

| Model               | Original | INT8   | FP16  | Speed (INT8) | Accuracy Loss    |
| ------------------- | -------- | ------ | ----- | ------------ | ---------------- |
| YOLOv8n (24 MB)     | 24 MB    | 6 MB   | 12 MB | ~1.5x faster | < 0.5% mAP       |
| ResNet50 (98 MB)    | 98 MB    | 25 MB  | 49 MB | ~2x faster   | < 1% top-1 acc   |
| MobileNetV2 (14 MB) | 14 MB    | 3.5 MB | 7 MB  | ~1.2x faster | < 0.3% top-1 acc |

**Process:**

1. Original model → load via ONNX Runtime
2. Calibration → if INT8, run representative dataset to collect activation statistics
3. Quantization → convert weights/activations to lower precision
4. Optimization → fuse operations, fold constants, select best kernels
5. Serialization → write as `.ort` file with metadata
6. Bundling → embed in binary or package separately

### ORT Performance & Hardware Acceleration

| Provider               | Hardware             | Latency              | Throughput            |
| ---------------------- | -------------------- | -------------------- | --------------------- |
| **CPU (ONNX Runtime)** | ARM Cortex-A         | ~100–500 ms (YOLOv8) | 2–10 inferences/sec   |
| **GPU (CUDA)**         | NVIDIA               | ~10–50 ms            | 20–100 inferences/sec |
| **NPU (OpenVINO)**     | Intel Movidius / VPU | ~20–100 ms           | 10–50 inferences/sec  |
| **DSP (QNN)**          | Qualcomm Hexagon     | ~50–200 ms           | 5–20 inferences/sec   |

**Selection logic:** Auto-detect available hardware; fall back to CPU if necessary.

### ORT Features

- **Automatic quantization:** Int8, FP16 support with accuracy preservation
- **Model optimization:** Operator fusion, constant folding, pruning
- **Batching:** Automatic batching for throughput optimization
- **Caching:** Session / intermediate result caching
- **Monitoring:** Inference time, memory usage, cache hit rates
