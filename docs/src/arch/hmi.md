## Consortium HMI: Human-Machine Interface

The Consortium HMI provides a unified framework for building responsive, real-time UIs on the A-core Linux side, with automatic backend integration and multi-protocol support (web, embedded GUI, mobile).

### HMI Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Consortium Application (A-Core Linux)                      │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Tokio Runtime                                        │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │  HMI Event Loop                                 │  │  │
│  │  │  ws_listen → recv_events → update_state         │  │  │
│  │  │              ↓                                   │  │  │
│  │  │  Render Template (Tera/Askama)                  │  │  │
│  │  │              ↓                                   │  │  │
│  │  │  Broadcast to WebSocket Clients                 │  │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │  State Management                               │  │  │
│  │  │  • System status (temps, voltages, frequencies) │  │  │
│  │  │  • Device state (on/off, mode, settings)        │  │  │
│  │  │  • User preferences (theme, language, etc.)     │  │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  └────────┬────────────────────────────────────────────────┘  │
│           │                                                    │
│  ┌────────┴────────────────────────────────────────────────┐  │
│  │  IPC / TEE / ORT Integration                           │  │
│  │  • Fetch sensor data via IPC (M-core)                  │  │
│  │  • Get auth status via TEE (A-core)                    │  │
│  │  • Run inference via ORT (AI model)                    │  │
│  └──────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
        │              │              │
        ▼              ▼              ▼
    ┌────────┐   ┌─────────┐   ┌───────────┐
    │ WebGTK │   │ Browser │   │ Mobile    │
    │ (local)│   │ (remote)│   │ (Tauri)   │
    └────────┘   └─────────┘   └───────────┘
```

### Developer Experience: Declarative UI + Reactive State

#### Step 1: Define State & Events

State in the HMI is the single source of truth. All UI updates derive from state changes.

```rust
// my-app/src/hmi/state.rs
use serde::{Deserialize, Serialize};
use consortium_hmi::State;

/// Application state (serializable for persistence/sync)
#[derive(State, Clone, Debug, Serialize, Deserialize)]
pub struct AppState {
    // Sensor data (from M-core via IPC)
    pub temperature: f32,
    pub humidity: f32,
    pub pressure_hpa: f32,

    // Device control
    pub device_mode: DeviceMode,
    pub target_temperature: f32,
    pub heating_enabled: bool,

    // User session
    pub is_authenticated: bool,
    pub username: String,
    pub user_role: UserRole,

    // UI state
    pub selected_tab: TabId,
    pub show_advanced_settings: bool,
    pub notification_queue: Vec<Notification>,

    // Metrics
    pub uptime_seconds: u64,
    pub last_update_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeviceMode {
    Idle,
    Heating { target: f32 },
    Cooling { target: f32 },
    Auto,
    Error { reason: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    Guest,
    User,
    Administrator,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TabId {
    Overview,
    Control,
    Settings,
    Analytics,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    pub level: NotificationLevel,
    pub message: String,
    pub timestamp_ms: u64,
    pub dismissible: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            temperature: 20.0,
            humidity: 50.0,
            pressure_hpa: 1013.25,
            device_mode: DeviceMode::Idle,
            target_temperature: 22.0,
            heating_enabled: false,
            is_authenticated: false,
            username: String::new(),
            user_role: UserRole::Guest,
            selected_tab: TabId::Overview,
            show_advanced_settings: false,
            notification_queue: Vec::new(),
            uptime_seconds: 0,
            last_update_ms: 0,
        }
    }
}
```

#### Step 2: Define Events (User Interactions)

Events are actions triggered by user interaction or external systems. They mutate the state.

```rust
// my-app/src/hmi/events.rs
use consortium_hmi::Event;

/// All possible user interactions and external triggers
#[derive(Event, Clone, Debug)]
pub enum AppEvent {
    // Sensor data updates (from M-core)
    SensorData {
        temperature: f32,
        humidity: f32,
        pressure_hpa: f32,
    },

    // User interactions
    SetMode(DeviceMode),
    SetTargetTemperature(f32),
    ToggleHeating,
    SelectTab(TabId),
    ShowAdvancedSettings(bool),

    // Authentication
    Login { username: String, password: String },
    Logout,

    // System events
    SystemHealthCheck,
    ClearNotification(usize),
    AddNotification(Notification),

    // Periodic updates
    Tick { elapsed_ms: u64 },
}

impl AppEvent {
    pub fn describe(&self) -> String {
        match self {
            Self::SensorData { temperature, .. } => {
                format!("Sensor update: temp={:.1}°C", temperature)
            }
            Self::SetMode(mode) => format!("Device mode: {:?}", mode),
            Self::Login { username, .. } => format!("Login: {}", username),
            _ => format!("{:?}", self),
        }
    }
}
```

#### Step 3: Define HMI Handler with Full State Management

```rust
// my-app/src/hmi/handler.rs
use consortium_hmi::hmi;
use crate::hmi::{AppState, AppEvent, Notification, NotificationLevel};

#[hmi]
pub struct AppHmi {
    template_cache: std::sync::Arc<tokio::sync::Mutex<TemplateCache>>,
}

impl AppHmi {
    pub fn new() -> Self {
        Self {
            template_cache: std::sync::Arc::new(tokio::sync::Mutex::new(TemplateCache::new())),
        }
    }

    /// Render current state to HTML (< 50ms typical)
    pub async fn render(&self, state: &AppState) -> Result<String> {
        use tera::{Tera, Context};

        let mut tera = Tera::new("templates/**/*.html")?;
        let mut context = Context::new();
        context.insert("state", &state);

        let template_name = match state.selected_tab {
            TabId::Overview => "overview.html",
            TabId::Control => "control.html",
            TabId::Settings => "settings.html",
            TabId::Analytics => "analytics.html",
        };

        tera.render(template_name, &context)
            .map_err(|e| format!("Render error: {}", e).into())
    }

    /// Process event and update state atomically
    pub async fn on_event(
        &self,
        event: AppEvent,
        state: &mut AppState,
        ipc: &mut IpcChannels,
        tee: &TeeSession,
        ort: &ModelPool,
    ) -> Result<bool> {
        let old_state = state.clone();

        match event {
            // Sensor data with optional ML
            AppEvent::SensorData { temperature, humidity, pressure_hpa } => {
                state.temperature = temperature;
                state.humidity = humidity;
                state.pressure_hpa = pressure_hpa;

                if state.is_authenticated && temperature > 30.0 {
                    let prediction = ort.predict(&[temperature, humidity, pressure_hpa]).await?;
                    if prediction.anomaly_score > 0.8 {
                        state.notification_queue.push(Notification {
                            level: NotificationLevel::Warning,
                            message: "Temperature anomaly detected".to_string(),
                            timestamp_ms: state.last_update_ms,
                            dismissible: true,
                        });
                    }
                }
            }

            // Device control with authentication check
            AppEvent::SetMode(mode) => {
                if !state.is_authenticated && !matches!(mode, DeviceMode::Idle) {
                    state.notification_queue.push(Notification {
                        level: NotificationLevel::Error,
                        message: "Authentication required".to_string(),
                        timestamp_ms: state.last_update_ms,
                        dismissible: true,
                    });
                    return Ok(false);
                }

                state.device_mode = mode.clone();
                let cmd = match mode {
                    DeviceMode::Heating { target } => {
                        Command::SetHeating { enabled: true, target_temp: target }
                    }
                    DeviceMode::Cooling { target } => {
                        Command::SetCooling { enabled: true, target_temp: target }
                    }
                    DeviceMode::Idle => Command::SetIdle,
                    _ => return Ok(false),
                };

                ipc.cmd_channel.send(&cmd).await?;
            }

            // Temperature validation
            AppEvent::SetTargetTemperature(target) => {
                if target < 15.0 || target > 35.0 {
                    state.notification_queue.push(Notification {
                        level: NotificationLevel::Error,
                        message: "Temperature must be 15–35°C".to_string(),
                        timestamp_ms: state.last_update_ms,
                        dismissible: true,
                    });
                    return Ok(false);
                }
                state.target_temperature = target;
                ipc.cmd_channel.send(&Command::SetTargetTemp(target)).await?;
            }

            // TEE-based authentication
            AppEvent::Login { username, password } => {
                match tee.call_authenticate(&username, password.as_bytes()).await {
                    Ok(auth_result) if auth_result.valid => {
                        state.is_authenticated = true;
                        state.username = username.clone();
                        state.user_role = auth_result.role;

                        state.notification_queue.push(Notification {
                            level: NotificationLevel::Info,
                            message: format!("Welcome, {}!", username),
                            timestamp_ms: state.last_update_ms,
                            dismissible: false,
                        });

                        if !matches!(state.user_role, UserRole::Administrator) {
                            state.show_advanced_settings = false;
                        }
                    }
                    _ => {
                        state.notification_queue.push(Notification {
                            level: NotificationLevel::Error,
                            message: "Invalid credentials".to_string(),
                            timestamp_ms: state.last_update_ms,
                            dismissible: true,
                        });
                    }
                }
            }

            AppEvent::SelectTab(tab) => {
                state.selected_tab = tab;
            }

            AppEvent::Tick { elapsed_ms } => {
                state.uptime_seconds += 1;
                state.last_update_ms = elapsed_ms;
                state.notification_queue.retain(|n| {
                    elapsed_ms - n.timestamp_ms < 10_000 || !n.dismissible
                });
            }

            _ => {}
        }

        Ok(old_state != *state)
    }
}
```

#### Step 4: Complete Integration in Main App

```rust
// my-app/src/main.rs
use my_app::hmi::{AppHmi, AppState, AppEvent};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    // 1. Setup runtime
    let remoteproc = RemoteProc::new(0);
    remoteproc.set_firmware("firmware.elf")?;
    remoteproc.start()?;

    let uio = UioDevice::open(0).await?;
    let tee = TeeSession::open().await?;

    // 2. Initialize state
    let mut state = AppState::default();
    let hmi = AppHmi::new();
    let (state_tx, _) = broadcast::channel(100);
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel(1000);

    // 3. Task A: Listen for sensor updates from M-core
    let event_tx_clone = event_tx.clone();
    tokio::spawn(async move {
        let mut sensor_ch = uio.channel(1).await.unwrap();
        loop {
            match sensor_ch.recv::<SensorReading>().await {
                Ok(reading) => {
                    let event = AppEvent::SensorData {
                        temperature: reading.temperature,
                        humidity: reading.humidity,
                        pressure_hpa: reading.pressure,
                    };
                    let _ = event_tx_clone.send(event).await;
                }
                Err(e) => {
                    tracing::error!("Sensor error: {}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    });

    // 4. Task B: Periodic tick
    let event_tx_clone = event_tx.clone();
    tokio::spawn(async move {
        let start = std::time::Instant::now();
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let elapsed_ms = start.elapsed().as_millis() as u64;
            let _ = event_tx_clone.send(AppEvent::Tick { elapsed_ms }).await;
        }
    });

    // 5. Task C: WebSocket server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    let state_tx_clone = state_tx.clone();
    let event_tx_clone = event_tx.clone();

    tokio::spawn(async move {
        loop {
            if let Ok((stream, peer_addr)) = listener.accept().await {
                tracing::info!("Client: {}", peer_addr);
                let state_tx = state_tx_clone.clone();
                let event_tx = event_tx_clone.clone();

                tokio::spawn(async move {
                    let _ = handle_websocket_client(stream, peer_addr, state_tx, event_tx).await;
                });
            }
        }
    });

    // 6. Main event loop
    while let Some(event) = event_rx.recv().await {
        tracing::debug!("Event: {}", event.describe());

        match hmi.on_event(event, &mut state, &mut IpcChannels {…}, &tee, &ort_pool).await {
            Ok(state_changed) if state_changed => {
                match hmi.render(&state).await {
                    Ok(html) => { let _ = state_tx.send(html); }
                    Err(e) => tracing::error!("Render: {}", e),
                }
            }
            Err(e) => {
                tracing::error!("Error: {}", e);
                state.notification_queue.push(Notification {
                    level: NotificationLevel::Error,
                    message: format!("Error: {}", e),
                    timestamp_ms: 0,
                    dismissible: true,
                });
            }
            _ => {}
        }
    }

    remoteproc.stop()?;
    Ok(())
}

async fn handle_websocket_client(
    stream: tokio::net::TcpStream,
    peer_addr: std::net::SocketAddr,
    state_tx: broadcast::Sender<String>,
    event_tx: tokio::sync::mpsc::Sender<AppEvent>,
) -> Result<()> {
    let ws = tokio_tungstenite::accept_async(stream).await?;
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut state_rx = state_tx.subscribe();

    loop {
        select! {
            msg = state_rx.recv() => {
                match msg {
                    Ok(html) => ws_tx.send(Message::Text(html)).await?,
                    Err(broadcast::error::RecvError::Lagged(_)) => {},
                    Err(e) => break,
                }
            }

            msg = ws_rx.next() => {
                match msg {
                    Some(Ok(Message::Text(json))) => {
                        if let Ok(event) = serde_json::from_str::<AppEvent>(&json) {
                            let _ = event_tx.send(event).await;
                        }
                    }
                    Some(Ok(Message::Close(_))) => break,
                    Some(Err(_)) => break,
                    None => break,
                }
            }
        }
    }

    tracing::info!("Client closed: {}", peer_addr);
    Ok(())
}
```

#### Step 5: Full HTML Template with Styling & Interactivity

```html
<!-- templates/overview.html -->
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Consortium Control</title>
    <style>
      * {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
      }
      body {
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        min-height: 100vh;
        padding: 20px;
      }
      .container {
        max-width: 1200px;
        margin: 0 auto;
      }
      .header {
        color: white;
        margin-bottom: 30px;
      }
      .header h1 {
        font-size: 2.5em;
        margin-bottom: 10px;
      }

      .grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 20px;
      }
      .card {
        background: white;
        border-radius: 12px;
        padding: 24px;
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
        transition: transform 0.2s;
      }
      .card:hover {
        transform: translateY(-5px);
      }

      .sensor-grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 15px;
      }
      .sensor {
        text-align: center;
        padding: 15px;
        background: #f8f9fa;
        border-radius: 8px;
      }
      .sensor-value {
        font-size: 2.5em;
        font-weight: bold;
        color: #667eea;
      }

      button {
        padding: 12px 24px;
        border: none;
        border-radius: 8px;
        font-weight: 600;
        cursor: pointer;
        background: #667eea;
        color: white;
        transition: all 0.2s;
      }
      button:hover {
        background: #5568d3;
      }

      .notification {
        position: fixed;
        top: 20px;
        right: 20px;
        padding: 15px;
        background: white;
        border-radius: 8px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
      }
    </style>
  </head>
  <body>
    <div class="container">
      <div class="header">
        <h1>🏠 Environmental Control</h1>
      </div>

      <div class="grid">
        <div class="card">
          <h2>📊 Sensors</h2>
          <div class="sensor-grid">
            <div class="sensor">
              <div>Temperature</div>
              <div class="sensor-value">{{ state.temperature | round(precision=1) }}°C</div>
            </div>
            <div class="sensor">
              <div>Humidity</div>
              <div class="sensor-value">{{ state.humidity | round }}%</div>
            </div>
          </div>
        </div>

        <div class="card">
          <h2>🎛️ Device</h2>
          <p>
            Mode:
            <strong
              >{% match state.device_mode %} {% when DeviceMode::Idle %}Idle {% when
              DeviceMode::Heating {target} %}Heating to {{ target }}°C {% endmatch %}</strong
            >
          </p>
          <button onclick="send('SetMode', 'Heating')">Heat</button>
          <button onclick="send('SetMode', 'Idle')">Off</button>
        </div>
      </div>
    </div>

    <script>
      const ws = new WebSocket("ws://" + location.host + "/ws");
      ws.onmessage = (e) => (document.documentElement.innerHTML = e.data);

      function send(action, value) {
        const event = {};
        event[action] = value;
        ws.send(JSON.stringify(event));
      }
    </script>
  </body>
</html>
```

### HMI Key Insights

1. **State is immutable from render perspective:** Templates are pure functions of state
2. **Event-driven:** All mutations go through the event processor
3. **Broadcast pattern:** State changes sent to all connected clients instantly
4. **Error handling:** User-friendly notifications for all error conditions
5. **Integration:** Seamlessly connects IPC + TEE + ORT

### HMI Features

| Feature                 | Implementation                                 |
| ----------------------- | ---------------------------------------------- |
| **Template Engine**     | Tera/Askama for HTML generation                |
| **State Management**    | Tokio channels + broadcast                     |
| **WebSocket Transport** | tokio-tungstenite for real-time updates        |
| **Multi-Protocol**      | HTTP, WebSocket, optional Tauri for desktop    |
| **Responsive Design**   | CSS Grid/Flexbox, mobile-first                 |
| **Accessibility**       | WCAG 2.1 AA semantic HTML                      |
| **Customization**       | CSS variables, dark mode, internationalization |

### HMI Performance

- **State update latency:** < 100 ms (WebSocket broadcast)
- **WebSocket message size:** ~1–10 KB per update
- **Concurrent clients:** 100+ (typical)
- **Memory overhead:** ~1 MB per connected client
