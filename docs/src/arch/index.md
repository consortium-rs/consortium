# Consortium Architecture

The Consortium is a asymmetric multiprocessing framework that cooperates non-secure Linux, secure OP-TEE, non-secure RTOS, and secure RTOS. The architecture of the Consortium is designed to provide a secure and efficient communication channel between the different components while maintaining the security and isolation of each component.

Traditionally, developing heterogeneous and asymmetric multiprocessing systems, e.g., NXP's i.MX 8 series and ST's STM32MP1 series, requires significant engineering effort to design and implement the communication channel between the different components. The Consortium aims to simplify this process by providing a unified framework that abstracts away the complexities of inter-component communication.

The Consortium architecture consists of the following components:

- **Consortium IPC**: A communication channel that allows the different components to communicate with each other securely and efficiently. It provides a unified API for sending and receiving messages between the components, abstracting away the underlying communication mechanisms. Asynchronous and type-safe is the default choice. We implement hardware-specific IPC mechanisms, e.g., HSEM in STM32MP series and MU in i.MX series, as IPC backends. You only need to write `.send().await?` and `.recv().await?` to send and receive messages, and the IPC layer will take care of the rest, including serialization, deserialization, and synchronization.
- **Consortium TEE**: A secure execution environment that runs on the OP-TEE OS. It provides a secure environment for executing sensitive code and handling sensitive data. The Consortium TEE is designed to be compatible with the GlobalPlatform TEE specifications, allowing developers to leverage existing TEE applications and libraries. It uses macro-based code generation to simplify the development of TEE applications and ensure type safety. You only need to write it like a normal Rust library, and the macro will generate the necessary boilerplate code for you.
- **Consortium Runtime**: A runtime library that runs on the non-secure Linux and non-secure RTOS. It provides a unified API for interacting with the Consortium IPC and Consortium TEE, allowing developers to easily integrate the Consortium into their applications. The Consortium Runtime is designed to be lightweight and efficient, minimizing the overhead of using the Consortium in your applications.
- **Consortium HMI**: A human-machine interface (HMI) library that provides a unified API for creating user interfaces on the non-secure Linux side. It abstracts away the complexities of different GUI frameworks and allows developers to create user interfaces using a simple and consistent API. The Consortium HMI is designed to be flexible and extensible, allowing developers to easily integrate it with their existing applications and frameworks. It allows whatever GUI framework you want, especially web-based GUI frameworks. We can bundle a WebGTK automatically, and you can also view the page via a browser on another device, e.g., your phone, to interact with the Consortium system. You only need to write the UI logic, and the HMI layer will take care of the rest, including rendering and event handling.
- **Consortium FFI**: A foreign function interface (FFI) library that provides a unified API for calling functions across the different components. It allows developers to call functions in Python to run OpenCV for computer vision tasks, call functions in C to run TensorRT for AI inference tasks, etc. Though we recommend using Rust for the best experience, you can still use other languages if you want. The Consortium FFI is designed to be flexible and extensible, allowing developers to easily integrate it with their existing applications and frameworks.
- **Consortium ONNX Runtime**: A library that provides a wrapper of `ort` for running ONNX models in the Consortium system. It allows developers to easily run ONNX models on the different components, leveraging the hardware acceleration capabilities of each component. The Consortium ONNX Runtime is designed to be efficient and easy to use, allowing developers to quickly integrate ONNX model inference into their applications. We also automatically bundle the runtime library and the model together, so you can just drop the model file into a specific directory and start using it without worrying about the details of loading the model and running inference.
- **Consortium Developer Tools**: A set of tools that assist developers in building, testing, and debugging their applications using the Consortium framework. These tools include a build system that simplifies the process of building and deploying applications to the different components, a testing framework that allows developers to easily write and run tests for their applications, and a debugging tool that provides insights into the communication between the different components.

The Consortium architecture is designed to be modular and extensible, allowing developers to easily add new components and features as needed. By providing a unified framework for inter-component communication and secure execution, the Consortium aims to simplify the development of heterogeneous and asymmetric multiprocessing systems, enabling developers to focus on building innovative applications rather than dealing with the complexities of inter-component communication.


## Integration: Complete Example

Here's how TEE, HMI, ORT, and FFI work together:

```rust
// my-app/src/main.rs
use consortium_runtime_app::RemoteProc;
use consortium_tee::TeeSession;
use consortium_ort::ModelPool;
use consortium_ffi::FfiPool;
use my_app::models::ObjectDetectionModel;
use my_app::hmi::AppHmi;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize all subsystems
    let remoteproc = RemoteProc::new(0);
    remoteproc.start()?;

    let tee = TeeSession::open().await?;
    let od_model = ObjectDetectionModel::load("yolo.onnx").await?;
    let ffi_pool = FfiPool::new()
        .add_python_worker("cv_workers", 4)?
        .start()
        .await?;

    let hmi = AppHmi;

    // 2. Incoming image stream (from sensor via IPC or camera)
    let mut image_stream = create_image_stream().await?;

    while let Some(image) = image_stream.next().await {
        // Step A: Run ML inference
        let detections = od_model.predict(&image).await?;

        // Step B: Call Python for post-processing
        let edges = ffi_pool.detect_edges(image.to_bytes()).await?;

        // Step C: Verify authenticity (in TEE)
        let is_legit = tee.call_verify_image_signature(&edges).await?;

        // Step D: Update HMI with results
        let event = AppEvent::ObjectsDetected {
            detections,
            verified: is_legit,
        };
        broadcast_event(event).await?;
    }

    Ok(())
}
```

This demonstrates the power of the Consortium framework: developers can seamlessly combine:
- **IPC** for low-latency sensor data
- **TEE** for security-critical operations
- **ORT** for AI inference
- **FFI** for specialized libraries
- **HMI** for user interaction

All with a clean, ergonomic Rust API.
