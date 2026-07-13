## Consortium FFI: Foreign Function Interface

The Consortium FFI provides a unified interface for calling functions in Python, C, C++, and other languages from Rust code, with automatic serialization, process pooling, and type safety.

### FFI Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Consortium Application (Rust)                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Rust Code                                            │  │
│  │  let result = python_fn("process", input).await?      │  │
│  │  let cv_result = cpp_fn("detect_edges", image).await? │  │
│  └────────┬────────────────────────────────────────────┘  │
│           │                                                 │
│  ┌────────▼────────────────────────────────────────────┐  │
│  │  Consortium FFI Layer                               │  │
│  │  ├─ Function registry (Python/C/C++)                │  │
│  │  ├─ RPC marshaler (pickle, protobuf, JSON)          │  │
│  │  ├─ Process pool (worker management)                │  │
│  │  └─ Error translation                               │  │
│  └────────┬────────────────────────────────────────────┘  │
│           │                                                 │
│  ┌────────▼────────────────────────────────────────────┐  │
│  │  IPC Transport (stdio, socket, shared memory)        │  │
│  └────────┬────────────────────────────────────────────┘  │
│           │                                                 │
│  ┌────────▼────────────────────────────────────────────┐  │
│  │  Worker Processes                                   │  │
│  │  ├─ Python subprocess (with OpenCV, TensorFlow)     │  │
│  │  ├─ C subprocess (with libfoo)                       │  │
│  │  └─ C++ subprocess (with custom algorithms)         │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Developer Experience: Function Proxies

#### Step 1: Define Worker Services (Python Side)

Worker processes expose functions via JSON-RPC or binary protocols. They run independently and communicate via sockets/stdio.

```python
# ffi_workers/python_cv/main.py
"""
Computer Vision worker: provides OpenCV-based functions.
Each function runs in response to RPC calls from Rust.
"""
import cv2
import numpy as np
import json
import sys
from typing import Dict, List, Any
from dataclasses import dataclass, asdict

# Utilities
@dataclass
class EdgeDetectionResult:
    edges: bytes  # encoded as base64 in JSON
    width: int
    height: int

@dataclass
class PoseKeypoint:
    x: float
    y: float
    confidence: float

@dataclass
class PoseEstimateResult:
    keypoints: List[PoseKeypoint]
    average_confidence: float

# Worker functions
def detect_edges(image_bytes: bytes, threshold1: int = 100, threshold2: int = 200) -> Dict:
    """
    Detect edges in image using Canny algorithm.

    Args:
        image_bytes: JPEG-encoded image as bytes
        threshold1: Lower Canny threshold
        threshold2: Upper Canny threshold

    Returns:
        Dict with edges (base64-encoded PNG), width, height
    """
    try:
        # 1. Decode image
        nparr = np.frombuffer(image_bytes, np.uint8)
        img = cv2.imdecode(nparr, cv2.IMREAD_COLOR)
        if img is None:
            raise ValueError("Failed to decode image")

        # 2. Convert to grayscale and detect edges
        gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
        edges = cv2.Canny(gray, threshold1, threshold2)

        # 3. Encode result
        success, buffer = cv2.imencode('.png', edges)
        if not success:
            raise ValueError("Failed to encode result")

        # 4. Return as base64 for JSON serialization
        import base64
        edges_b64 = base64.b64encode(buffer).decode('utf-8')

        return {
            "status": "ok",
            "result": {
                "edges": edges_b64,
                "width": int(edges.shape[1]),
                "height": int(edges.shape[0]),
            }
        }
    except Exception as e:
        return {"status": "error", "error": str(e)}

def estimate_pose(image_bytes: bytes, model_path: str) -> Dict:
    """
    Estimate human pose using MediaPipe.

    Args:
        image_bytes: JPEG-encoded image
        model_path: Path to pose estimation model

    Returns:
        Dict with keypoints and confidence
    """
    try:
        import mediapipe as mp

        # 1. Decode image
        nparr = np.frombuffer(image_bytes, np.uint8)
        img = cv2.imdecode(nparr, cv2.IMREAD_COLOR)
        img_rgb = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)

        # 2. Run pose detection
        mp_pose = mp.solutions.pose
        with mp_pose.Pose() as pose:
            results = pose.process(img_rgb)

        if not results.pose_landmarks:
            return {"status": "ok", "result": {"keypoints": [], "average_confidence": 0.0}}

        # 3. Extract keypoints
        keypoints = []
        confidences = []
        for landmark in results.pose_landmarks.landmark:
            keypoints.append({
                "x": float(landmark.x),
                "y": float(landmark.y),
                "confidence": float(landmark.visibility),
            })
            confidences.append(float(landmark.visibility))

        avg_conf = np.mean(confidences) if confidences else 0.0

        return {
            "status": "ok",
            "result": {
                "keypoints": keypoints,
                "average_confidence": float(avg_conf),
            }
        }
    except Exception as e:
        return {"status": "error", "error": str(e)}

# Worker server loop
if __name__ == "__main__":
    import json

    # Register available functions
    FUNCTIONS = {
        "detect_edges": detect_edges,
        "estimate_pose": estimate_pose,
    }

    while True:
        try:
            # Read JSON-RPC request from stdin
            line = input()
            request = json.loads(line)

            method = request.get("method")
            params = request.get("params", {})
            request_id = request.get("id")

            if method not in FUNCTIONS:
                response = {
                    "id": request_id,
                    "error": f"Unknown method: {method}"
                }
            else:
                # Call function
                result = FUNCTIONS[method](**params)
                response = {
                    "id": request_id,
                    "result": result
                }

            # Send response
            print(json.dumps(response))
            sys.stdout.flush()

        except EOFError:
            break
        except Exception as e:
            sys.stderr.write(f"Worker error: {e}\n")
            sys.stderr.flush()
```

#### Step 2: Define Rust Interface & RPC Proxies

```rust
// my-app/src/ffi/mod.rs
use serde::{Deserialize, Serialize};
use consortium_ffi::{FfiPool, RemoteCallError};

// Define return types that match Python functions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EdgeDetectionResult {
    pub edges: Vec<u8>,  // decoded from base64
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoseKeypoint {
    pub x: f32,
    pub y: f32,
    pub confidence: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoseEstimateResult {
    pub keypoints: Vec<PoseKeypoint>,
    pub average_confidence: f32,
}

// FFI request/response wrapper
#[derive(Serialize, Deserialize)]
struct WorkerResponse<T> {
    status: String,
    result: Option<T>,
    error: Option<String>,
}

impl<T> WorkerResponse<T> {
    fn into_result(self) -> Result<T, RemoteCallError> {
        match self.status.as_str() {
            "ok" => self.result.ok_or_else(||
                RemoteCallError::InvalidResponse("Missing result".to_string())
            ),
            _ => Err(RemoteCallError::RemoteError(
                self.error.unwrap_or_else(|| "Unknown error".to_string())
            )),
        }
    }
}

/// Python CV worker functions
pub struct PythonCvWorker {
    pool: FfiPool,
}

impl PythonCvWorker {
    pub async fn new(num_workers: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = FfiPool::new()
            .add_python_worker("ffi_workers/python_cv", num_workers)?
            .start()
            .await?;

        Ok(Self { pool })
    }

    /// Detect edges in image
    pub async fn detect_edges(
        &self,
        image_bytes: Vec<u8>,
        threshold1: u32,
        threshold2: u32,
    ) -> Result<EdgeDetectionResult, RemoteCallError> {
        let params = serde_json::json!({
            "image_bytes": image_bytes,
            "threshold1": threshold1,
            "threshold2": threshold2,
        });

        let response: WorkerResponse<EdgeDetectionResult> =
            self.pool.call_python("detect_edges", params).await?;

        response.into_result()
    }

    /// Estimate pose in image
    pub async fn estimate_pose(
        &self,
        image_bytes: Vec<u8>,
        model_path: String,
    ) -> Result<PoseEstimateResult, RemoteCallError> {
        let params = serde_json::json!({
            "image_bytes": image_bytes,
            "model_path": model_path,
        });

        let response: WorkerResponse<PoseEstimateResult> =
            self.pool.call_python("estimate_pose", params).await?;

        response.into_result()
    }
}

// Error types
#[derive(Debug)]
pub enum RemoteCallError {
    Serialization(serde_json::Error),
    Transport(String),
    RemoteError(String),
    InvalidResponse(String),
    Timeout,
}

impl std::fmt::Display for RemoteCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serialization(e) => write!(f, "Serialization: {}", e),
            Self::Transport(e) => write!(f, "Transport: {}", e),
            Self::RemoteError(e) => write!(f, "Remote error: {}", e),
            Self::InvalidResponse(e) => write!(f, "Invalid response: {}", e),
            Self::Timeout => write!(f, "RPC timeout"),
        }
    }
}

impl std::error::Error for RemoteCallError {}
```

#### Step 3: Comprehensive Application Integration

```rust
// my-app/src/main.rs
use my_app::ffi::PythonCvWorker;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    // 1. Initialize FFI worker pool
    tracing::info!("Starting Python CV worker pool...");
    let cv_workers = PythonCvWorker::new(num_cpus::get()).await?;
    tracing::info!("Worker pool started");

    // 2. Load image
    let image_path = "test_image.jpg";
    let image_bytes = std::fs::read(image_path)?;
    tracing::info!("Loaded image: {} bytes", image_bytes.len());

    // 3. Single call example
    {
        tracing::info!("Calling detect_edges...");
        let start = Instant::now();

        match cv_workers.detect_edges(image_bytes.clone(), 100, 200).await {
            Ok(result) => {
                let elapsed = start.elapsed();
                tracing::info!("✓ Edge detection: {}x{} (took {:?})", result.width, result.height, elapsed);

                // Save edges
                let edges_bytes = base64::decode(&result.edges)?;
                std::fs::write("edges.png", edges_bytes)?;
            }
            Err(e) => {
                tracing::error!("✗ Edge detection failed: {}", e);
            }
        }
    }

    // 4. Concurrent calls example
    {
        tracing::info!("Running concurrent calls...");
        let start = Instant::now();

        let (edges_result, pose_result) = tokio::join!(
            cv_workers.detect_edges(image_bytes.clone(), 100, 200),
            cv_workers.estimate_pose(image_bytes.clone(), "models/pose.pb".to_string()),
        );

        let elapsed = start.elapsed();

        match (edges_result, pose_result) {
            (Ok(edges), Ok(pose)) => {
                tracing::info!("✓ Concurrent calls completed in {:?}", elapsed);
                tracing::info!("  - Edges: {}x{}", edges.width, edges.height);
                tracing::info!("  - Pose: {} keypoints (avg conf: {:.2})", pose.keypoints.len(), pose.average_confidence);
            }
            (e1, e2) => {
                tracing::error!("✗ One or more calls failed: {:?}, {:?}", e1, e2);
            }
        }
    }

    // 5. Batch processing example
    {
        tracing::info!("Batch processing...");
        let image_paths = vec!["img1.jpg", "img2.jpg", "img3.jpg"];

        let mut tasks = Vec::new();
        for path in image_paths {
            let image = std::fs::read(path)?;
            let cv_clone = &cv_workers;

            tasks.push(async move {
                cv_clone.detect_edges(image, 100, 200).await
            });
        }

        let results = futures::future::join_all(tasks).await;
        let successful = results.iter().filter(|r| r.is_ok()).count();
        tracing::info!("Batch: {}/{} images processed", successful, results.len());
    }

    Ok(())
}
```

#### Step 4: Error Handling & Retry Logic

```rust
// my-app/src/ffi/retry.rs
use async_trait::async_trait;
use tokio::time::{sleep, Duration};

#[async_trait]
pub trait Retryable<T>: Sized {
    async fn with_retry(self, max_attempts: u32) -> Result<T, Box<dyn std::error::Error>>;
}

pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_backoff: Duration,
    pub max_backoff: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(5),
        }
    }
}

impl<T, E: 'static + std::error::Error> Retryable<T> for Result<T, E> {
    async fn with_retry(self, max_attempts: u32) -> Result<T, Box<dyn std::error::Error>> {
        let mut attempt = 0;
        let mut backoff = Duration::from_millis(100);

        loop {
            attempt += 1;

            match self {
                Ok(value) => return Ok(value),
                Err(e) if attempt >= max_attempts => {
                    return Err(Box::new(e) as Box<dyn std::error::Error>);
                }
                Err(e) => {
                    tracing::warn!(
                        "Attempt {} failed: {}. Retrying in {:?}...",
                        attempt, e, backoff
                    );
                    sleep(backoff).await;
                    backoff = (backoff * 2).min(Duration::from_secs(5));
                }
            }
        }
    }
}

// Usage in application
async fn detect_edges_with_retry(
    cv: &PythonCvWorker,
    image: Vec<u8>,
) -> Result<EdgeDetectionResult, Box<dyn std::error::Error>> {
    let mut attempt = 0;
    let max_attempts = 3;

    loop {
        attempt += 1;
        match cv.detect_edges(image.clone(), 100, 200).await {
            Ok(result) => return Ok(result),
            Err(e) if attempt >= max_attempts => {
                return Err(format!("Failed after {} attempts: {}", max_attempts, e).into());
            }
            Err(e) => {
                tracing::warn!("Attempt {}: {}, retrying...", attempt, e);
                sleep(Duration::from_millis(100 * attempt as u64)).await;
            }
        }
    }
}
```

### FFI Best Practices

**1. Function Design:**

- Keep functions pure (no side effects)
- Serialize/deserialize at boundaries only
- Batch operations where possible

**2. Performance:**

- Use binary serialization (postcard) for large data
- Pool workers to amortize process creation cost
- Cache expensive initialization (models, connections)

**3. Error Handling:**

- Implement retry logic for transient failures
- Log detailed errors for debugging
- Set timeouts for long-running calls

**4. Resource Management:**

- Limit worker pool size to available CPU cores
- Set memory limits per worker
- Clean up resources on shutdown

### FFI Features

| Feature                 | Implementation                            |
| ----------------------- | ----------------------------------------- |
| **Multi-language**      | Python, C, C++, via subprocess            |
| **Serialization**       | postcard, JSON, protobuf (pluggable)      |
| **Process pooling**     | Work-stealing queue, load balancing       |
| **Error handling**      | Automatic translation, stack traces       |
| **Type safety**         | Compile-time type checking via macros     |
| **Async/await**         | Full Tokio integration                    |
| **Resource management** | Automatic cleanup, memory limits          |
| **Monitoring**          | Execution time, memory usage, error rates |

### FFI Performance

**Latency breakdown (Python function call):**
| Component | Time |
|-----------|------|
| Serialization | ~0.1–1 ms |
| IPC (socket) | ~0.5–2 ms |
| Python deserialization | ~0.1–0.5 ms |
| Function execution | _variable_ |
| Python serialization | ~0.1–0.5 ms |
| IPC response | ~0.5–2 ms |
| Rust deserialization | ~0.1–1 ms |
| **Total overhead** | **~2–7 ms** |

**Optimization strategies:**

- Keep functions long-running (amortize overhead)
- Batch multiple calls where possible
- Use binary serialization (postcard) over JSON
- Pool workers to avoid process spawn overhead
