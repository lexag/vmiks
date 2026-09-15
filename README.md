# Offline Multicam Vision Mixer — Project Roadmap

## 0. Project Definition

### Goal

Build a lightweight, open-source desktop application for editing prerecorded multicamera theatre performances using a **live vision-mixer workflow**.

The operator should be able to:

1. Load 4–8 prerecorded camera files.
2. Manually specify the synchronization offset of every camera.
3. Play all cameras against a common show clock.
4. See all cameras simultaneously in a multiview.
5. Select a camera as Preview.
6. Cut or transition to it as Program using keyboard shortcuts or UI buttons.
7. Record every switching decision against the show clock.
8. Play back the resulting program edit.
9. Modify/delete/re-record switching decisions.
10. Render the final program from the original camera files.
11. Optionally export the edit as an EDL/FCPXML/other interchange format.

### Non-goals

The application is NOT intended to be:

- A general-purpose NLE.
- A colour grading application.
- A compositing application.
- A title/graphics editor.
- An audio editor.
- A live broadcast system.
- An automatic multicam editor.
- An automatic synchronization system.
- A replacement for DaVinci Resolve.

The application should intentionally remain small and focused.

---

# 1. Core Concept

The fundamental abstraction is the **Show Timeline**.

Every camera has:

- a media file
- a source timeline
- an offset relative to show time

The application maintains one master clock:

    show_time = 0.0 ... duration

For each camera:

    source_time = show_time - camera.offset

If:

    camera.offset = 12.5

then:

    show time 12.5 -> source time 0.0

Before source_time reaches zero, the camera is considered unavailable.

Example:

                    SHOW TIME
    0:00                5:00               10:00
     │                   │                   │
     ├───────────────────┼───────────────────┤

    CAM 1 ├───────────────────────────────────
    CAM 2         ├───────────────────────────
    CAM 3 ├───────────────────────────────────
    CAM 4                 ├───────────────────

This allows cameras to start at arbitrary points without requiring matching
file timecodes.

---

# 2. Architecture

Use a modular architecture from the beginning.

Suggested high-level structure:

    ┌───────────────────────────────────────────────┐
    │                   egui UI                     │
    │                                               │
    │  Multiview  │ Program │ Controls │ Timeline   │
    └───────────────────────┬───────────────────────┘
                            │
                            ▼
    ┌───────────────────────────────────────────────┐
    │                Application State              │
    │                                               │
    │ Project │ Playback │ Switcher │ Decisions     │
    └───────────────────────┬───────────────────────┘
                            │
            ┌───────────────┼────────────────┐
            ▼               ▼                ▼
       Media Engine    Decision Engine    Renderer
            │               │                │
            ▼               ▼                ▼
         FFmpeg          Edit List        FFmpeg
       / decoder          / EDL           filters

Suggested Rust crates/components:

- `eframe`
- `egui`
- `egui_dock` or equivalent docking UI if desired
- `serde`
- `serde_json`
- `uuid`
- `rfd`
- `anyhow`
- `thiserror`
- `tracing`
- `tracing-subscriber`

Media playback should be isolated behind your own abstraction so the
rest of the application doesn't care whether the implementation uses:

- FFmpeg
- GStreamer
- libmpv
- another decoder/backend

Do NOT spread FFmpeg/GStreamer-specific types throughout the application.

---

# 3. Phase 1 — Project Skeleton

## Objective

Create a running egui desktop application with a clean internal architecture.

## Tasks

Create a Cargo workspace or single crate.

Recommended initial modules:

    src/
        main.rs

        app/
            mod.rs
            state.rs

        project/
            mod.rs
            model.rs
            io.rs

        media/
            mod.rs
            source.rs
            metadata.rs
            player.rs

        playback/
            mod.rs
            clock.rs

        switcher/
            mod.rs
            state.rs
            decision.rs

        render/
            mod.rs

        ui/
            mod.rs
            multiview.rs
            program.rs
            controls.rs
            timeline.rs
            cameras.rs
            project.rs

        error.rs

## Initial application state

    struct AppState {
        project: Option<Project>,
        playback: PlaybackState,
        switcher: SwitcherState,
        decisions: DecisionList,
    }

Do not start by implementing video decoding.

First get the application state and UI architecture correct.

---

# 4. Phase 2 — Project Data Model

Create a serializable project format.

Suggested project file:

    .ovm
    Offline Vision Mixer project

Internally this can initially just be JSON.

Example:

    {
      "version": 1,
      "show": {
        "name": "Hamlet",
        "duration": 8123.4
      },
      "cameras": [
        {
          "id": "cam1",
          "name": "Camera 1",
          "path": "/media/cam1.mov",
          "offset": 0.0
        },
        {
          "id": "cam2",
          "name": "Camera 2",
          "path": "/media/cam2.mov",
          "offset": 3.217
        }
      ],
      "decisions": []
    }

## Camera model

    struct Camera {
        id: CameraId,
        name: String,
        path: PathBuf,
        offset: Duration,
        enabled: bool,
    }

Later:

    struct MediaInfo {
        duration: Duration,
        width: u32,
        height: u32,
        frame_rate: Rational,
        time_base: Rational,
        audio_channels: u32,
        codec: String,
    }

Do not assume all cameras have identical:

- resolution
- framerate
- codec
- audio configuration

Normalize these later during rendering.

---

# 5. Phase 3 — Media Inspection

Before playback, implement media probing.

The user selects a video file.

The application extracts:

- duration
- dimensions
- framerate
- codec
- pixel format
- audio properties
- time base

Display:

    Camera 1
    ─────────────────────────────
    3840 × 2160
    25 fps
    Duration: 02:14:32.120
    H.264
    Audio: 2ch

This phase is important because it gives you a reliable foundation for
later synchronization and rendering.

---

# 6. Phase 4 — Manual Camera Synchronization

Create a dedicated "Camera Setup" screen.

Example:

    Cameras
    ───────────────────────────────────────────

    Camera 1    00:00:00.000    [Set offset]
    Camera 2    00:02:13.440    [Set offset]
    Camera 3   -00:00:04.200    [Set offset]
    Camera 4    00:01:27.800    [Set offset]

Allow offsets to be entered as:

    HH:MM:SS.mmm

Internally store them as a signed duration.

## Important

Do not define synchronization in terms of "camera start time".

Define:

    camera_offset

as:

    show_time corresponding to source_time = 0

This makes the mathematics trivial.

    source_time = show_time - offset

## Visual synchronization aid

A camera preview should allow:

- play
- pause
- seek
- frame step
- set offset from current position

Eventually provide:

    "Set this frame as show 00:00:00"

or:

    "Align this camera to current show time"

But the first implementation only needs manual numeric offsets.

---

# 7. Phase 5 — Playback Clock

This is one of the most important pieces of the application.

Create a dedicated master clock.

    struct ShowClock {
        position: Duration,
        playing: bool,
        rate: f64,
    }

Do NOT derive the clock from the video decoder.

The clock is authoritative.

The media engine follows it.

When playing:

    show_time += elapsed_real_time * playback_rate

For every camera:

    source_time = show_time - camera.offset

This means every component agrees about where the show is.

## Required controls

- Play
- Pause
- Stop
- Seek
- Step forward one frame
- Step backward one frame
- Jump ±5 seconds
- Jump ±30 seconds
- Playback speed

Initially support:

    0.25x
    0.5x
    1x
    2x

Avoid reverse playback initially.

---

# 8. Phase 6 — Media Playback Engine

This should be treated as an independent subsystem.

Required abstraction:

    trait VideoSource {
        fn seek(&mut self, position: Duration);
        fn frame_at(&mut self, position: Duration) -> Result<VideoFrame>;
        fn duration(&self) -> Duration;
    }

For real-time playback:

    fn update(&mut self, target_time: Duration)

The media engine should maintain a decoded frame near the requested timestamp.

## MVP implementation

It is acceptable to implement this relatively inefficiently initially.

For example:

    show clock
        ↓
    requested source time
        ↓
    decoder
        ↓
    current frame

Then optimize.

Do not attempt to solve perfect frame-accurate random-access playback for every codec
on day one.

---

# 9. Phase 7 — Choose a Media Backend

Evaluate these approaches early:

## Option A — FFmpeg

Pros:

- extremely broad codec support
- excellent rendering capabilities
- mature
- good fit for final rendering

Cons:

- Rust integration complexity
- licensing/distribution considerations
- hardware acceleration can become complicated

## Option B — GStreamer

Pros:

- excellent streaming architecture
- hardware acceleration
- sophisticated pipelines
- strong seeking support

Cons:

- more complex dependency stack
- deployment complexity

## Option C — libmpv

Pros:

- mature video playback
- relatively easy playback abstraction
- hardware acceleration

Cons:

- less control over internals
- less attractive for eventual custom rendering pipeline

## Recommendation

Prototype playback with whichever backend gives you the fastest reliable
result.

Keep it behind:

    media::VideoDecoder

so the backend can be replaced later.

For final rendering, use FFmpeg regardless if practical.

---

# 10. Phase 8 — Multiview

Once one camera can play reliably, build the multiview.

For 4–8 cameras:

    ┌──────────┬──────────┬──────────┬──────────┐
    │ CAMERA 1 │ CAMERA 2 │ CAMERA 3 │ CAMERA 4 │
    │          │          │          │          │
    ├──────────┼──────────┼──────────┼──────────┤
    │ CAMERA 5 │ CAMERA 6 │ CAMERA 7 │ CAMERA 8 │
    │          │          │          │          │
    └──────────┴──────────┴──────────┴──────────┘

Each cell should show:

- video
- camera name
- camera number
- unavailable/offline indicator
- optional current source time

The currently selected Preview camera gets a visible border.

The current Program camera gets another visible indicator.

Avoid expensive UI decoration.

The video should dominate the screen.

---

# 11. Phase 9 — Program Monitor

Add a large Program monitor.

    ┌───────────────────────────────────────────┐
    │                                           │
    │                                           │
    │                 PROGRAM                   │
    │                                           │
    │                                           │
    └───────────────────────────────────────────┘

Below:

    Preview: CAM 3
    Program: CAM 1

This is conceptually identical to a broadcast switcher.

The Preview camera is the camera that will be taken next.

---

# 12. Phase 10 — Switcher State

Define:

    enum Transition {
        Cut,
        Dissolve {
            duration: Duration
        }
    }

And:

    struct SwitcherState {
        preview: CameraId,
        program: CameraId,
        transition: Transition,
    }

The basic operation:

    take(camera)

becomes a decision.

For a cut:

    current program ends
    new camera begins

For dissolve:

    current program and new camera overlap
    according to transition duration

---

# 13. Phase 11 — Decision List

This is the heart of the application.

Do NOT store "edits" as modified video.

Store decisions.

Simplest representation:

    struct Decision {
        time: Duration,
        camera: CameraId,
        transition: Transition,
    }

Example:

    [
        {
            "time": "00:00:00.000",
            "camera": "cam1",
            "transition": "cut"
        },
        {
            "time": "00:00:14.433",
            "camera": "cam3",
            "transition": "cut"
        },
        {
            "time": "00:00:31.210",
            "camera": "cam2",
            "transition": {
                "type": "dissolve",
                "duration": "00:00:00.500"
            }
        }
    ]

This becomes the canonical representation of the finished edit.

---

# 14. Phase 12 — Recording Switches

When the user presses:

    1
    2
    3
    4
    ...

the application changes Preview.

When the user presses:

    SPACE

perform TAKE.

Record:

    show_clock.position

not:

    system time
    decoder timestamp
    UI frame timestamp

This guarantees deterministic edits.

Example:

    Show time = 01:13:42.240

Operator presses:

    CAM 4
    TAKE

Decision:

    01:13:42.240 -> CAM4

---

# 15. Phase 13 — Keyboard-First Operation

This should be a major design goal.

Suggested bindings:

    1-8       Select camera
    SPACE     TAKE
    P         Play/Pause
    J         Reverse / eventually
    K         Pause
    L         Forward / eventually

    LEFT      Previous frame
    RIGHT     Next frame

    SHIFT+LEFT    -5 sec
    SHIFT+RIGHT   +5 sec

    CTRL+Z    Undo last decision

Potentially:

    Q/W       transition selection
    +/-       dissolve duration

The operator should be able to perform an entire show without touching
the mouse.

---

# 16. Phase 14 — First End-to-End Milestone

At this point the application should be able to:

1. Load 4 video files.
2. Assign offsets.
3. Play synchronized video.
4. Display all four cameras.
5. Select a camera.
6. Take it.
7. Record decisions.
8. Stop.
9. Replay the program.

This is the first **real product milestone**.

Do NOT start rendering yet.

Make this workflow solid first.

---

# 17. Phase 15 — Program Playback

Build a function:

    program_camera_at(time)

which evaluates the decision list.

Example:

    decisions:

    00:00 CAM1
    00:10 CAM2
    00:25 CAM3
    00:40 CAM1

Then:

    program_camera_at(00:05) -> CAM1
    program_camera_at(00:17) -> CAM2
    program_camera_at(00:30) -> CAM3

The Program monitor can therefore be driven entirely from:

    show_time + decision_list

This is important:

## The program monitor should NOT be a separately rendered video.

It should be a live evaluation of the decision list.

That means changing a decision instantly changes playback.

---

# 18. Phase 16 — Editing the Decision List

Add a simple timeline.

    00:00                    01:00
    │────────────────────────│

    CAM1 ███████
              CAM2 █████████
                         CAM4 █████
                                CAM1 ███████

Clicking a decision should select it.

Allow:

- move decision
- change camera
- delete decision
- insert decision
- change transition
- change transition duration

Keyboard shortcuts:

    Delete
    Ctrl+Z
    Ctrl+Shift+Z

The timeline does NOT need to become a full NLE.

It is simply a visualization/editor for the switch list.

---

# 19. Phase 17 — Re-recording

Support a particularly important workflow:

    Play show
       ↓
    Direct it
       ↓
    Review
       ↓
    "Redo from here"
       ↓
    Play again
       ↓
    Make different choices

When the operator starts re-recording at time T:

Option A:

Delete all decisions after T.

This is probably the simplest and most intuitive behavior.

Example:

    Existing:

    00:00 CAM1
    00:20 CAM2
    00:40 CAM3
    01:00 CAM4

    Re-record from 00:30

becomes:

    00:00 CAM1
    00:20 CAM2

Then new decisions are appended from 00:30 onward.

---

# 20. Phase 18 — Undo/Redo

Implement a proper command/history system.

Commands:

    TakeCamera
    MoveDecision
    DeleteDecision
    ChangeTransition
    ChangeCameraOffset

This will make undo/redo much easier.

Do not implement undo by copying the entire project state every time.

A command model will scale better.

---

# 21. Phase 19 — Rendering Architecture

Only after the decision system works should rendering begin.

The renderer should consume:

    Project
    +
    DecisionList

and produce:

    final video

Conceptually:

    camera files
         │
         ▼
    synchronization
         │
         ▼
    decision list
         │
         ▼
    render plan
         │
         ▼
       FFmpeg
         │
         ▼
    final program

---

# 22. Phase 20 — Generate an Intermediate Render Plan

Do NOT immediately generate one giant FFmpeg command.

First generate an internal representation:

    struct ProgramSegment {
        start: Duration,
        end: Duration,
        camera: CameraId,
        transition: Transition,
    }

Example:

    00:00 - 00:14 CAM1
    00:14 - 00:31 CAM3
    00:31 - 00:52 CAM2
    00:52 - 01:04 CAM4

Then rendering becomes a separate problem.

This also makes debugging vastly easier.

---

# 23. Phase 21 — Render Cuts

For cuts, the renderer can simply select ranges from source files.

For each segment:

    source_time =
        show_time - camera.offset

Therefore:

    CAM3 segment
    show:   00:14 -> 00:31
    offset: 00:05

becomes:

    source: 00:09 -> 00:26

The renderer then concatenates the resulting segments.

---

# 24. Phase 22 — Render Dissolves

A dissolve between:

    CAM1 -> CAM2

with duration:

    500 ms

requires overlap.

Conceptually:

    CAM1
    ───────────────────╲
                        ╲
                         ╲
                          ───────── CAM2
                         ╱
                        ╱
    ───────────────────╱

Generate the appropriate FFmpeg `xfade` or equivalent filter graph.

Do this after cuts work perfectly.

---

# 25. Phase 23 — Audio Strategy

This needs an explicit decision.

For theatre productions, the camera audio may not be what you ultimately want.

Possible future model:

    Program video
         +
    Master audio

For MVP:

## Option A

Use audio from a designated camera.

Simplest.

## Option B

Use a dedicated external audio file.

Probably much more useful for theatre.

Project model:

    audio:
        path: "FOH_mix.wav"
        offset: 0.0

Then the camera switching only affects video.

This is probably the correct long-term design.

---

# 26. Phase 24 — Audio Sync

Eventually allow:

    Master audio offset

so:

    audio_source_time = show_time - audio.offset

Again, no automatic synchronization is required.

---

# 27. Phase 25 — Export

Initial export targets:

    MP4 / H.264
    PCM/WAV or AAC audio

Later:

    ProRes
    DNxHR
    H.265
    image sequences

Expose presets rather than making the user understand FFmpeg.

Example:

    Export
    ────────────────────────────────

    Format:
      ● H.264 MP4
      ○ ProRes 422
      ○ DNxHR HQ

    Resolution:
      ● Source
      ○ 1080p
      ○ 720p

    Framerate:
      ● Project

    Audio:
      ● Master audio

    [ Render ]

---

# 28. Phase 26 — FCPXML Export

This is extremely valuable.

Instead of requiring your renderer to solve every possible codec problem,
allow:

    Project
       ↓
    Decision List
       ↓
    FCPXML
       ↓
    Resolve / Premiere / Final Cut

This gives the application immediate professional usefulness.

A theatre workflow could therefore be:

    Direct in Offline Vision Mixer
                 ↓
             FCPXML
                 ↓
             Resolve
                 ↓
       colour / audio / titles

The application becomes the specialized multicam directing front-end.

---

# 29. Phase 27 — EDL Export

Also support CMX3600 EDL where possible.

For simple cuts:

    001  CAM1  V  C  00:00:00:00 00:00:14:10 ...
    002  CAM3  V  C  00:00:14:10 00:00:31:05 ...

However, EDL has limitations for:

- dissolves
- multiple audio tracks
- complex source mapping

So consider FCPXML the more capable interchange format.

---

# 30. Phase 28 — Project Relinking

Absolute file paths are fragile.

Eventually store:

    path
    filename
    file size
    duration
    optional hash

If files move:

    Project opened
         ↓
    File not found
         ↓
    Search / Relink
         ↓
    Find matching media

For an MVP, just use absolute paths.

---

# 31. Phase 29 — Proxy Media

4–8 simultaneous 4K decoders may be too expensive.

Therefore introduce optional proxy files.

Project:

    Camera 1
        Original: cam1.mov
        Proxy:    proxies/cam1_720p.mp4

The operator uses proxies for:

    multiview
    program playback

Final rendering uses:

    original

This is probably essential for practical performance.

---

# 32. Phase 30 — Generate Proxies

Add:

    Media → Generate Proxies

Possible preset:

    1280 × 720
    H.264
    low bitrate

Display proxy status:

    CAM1   ✓
    CAM2   ✓
    CAM3   generating...
    CAM4   ✓

Do not block the UI while generating them.

---

# 33. Phase 31 — Performance Architecture

The biggest likely performance problem is decoding 4–8 streams simultaneously.

Do not make egui responsible for decoding.

Architecture:

    UI thread
         │
         ▼
    Playback coordinator
         │
       ┌─┴──────────────┐
       ▼                ▼
    Decoder 1        Decoder 2 ...
       │                │
       ▼                ▼
    Frame queue      Frame queue
       │                │
       └───────┬────────┘
               ▼
            egui/GPU

Use bounded queues.

Never allow decoding threads to grow indefinitely behind the UI.

---

# 34. Phase 32 — Frame Caching

Each camera should maintain a small frame cache.

At minimum:

    previous frame
    current frame
    next frame

For seeking:

    flush decoder
    seek to nearest keyframe
    decode forward

This should make normal playback responsive.

---

# 35. Phase 33 — GPU Upload

egui ultimately needs textures.

Do not recreate textures unnecessarily.

Use one GPU texture per visible camera.

Conceptually:

    decoded frame
          ↓
    pixel conversion
          ↓
    GPU texture update
          ↓
        egui

Eventually investigate:

- hardware decoding
- zero-copy paths
- NV12 textures
- Vulkan/Metal/DX12 integration

But these should be optimization phases, not MVP requirements.

---

# 36. Phase 34 — Timeline Interaction

The timeline should support:

- zoom
- horizontal scrolling
- playhead
- decision markers
- camera labels
- current program segment
- selection

Example:

    CAM1 ──────●────────────────────────────
               │
    CAM2       └────────●───────────────────
                        │
    CAM3                └──────●────────────

Playhead:

                 ▼
    ─────────────│─────────────────────────

Avoid building a general-purpose timeline editor.

It only needs to represent decisions.

---

# 37. Phase 35 — Transition Preview

For a transition at T:

    before T:
        program = CAM1

    after T:
        program = CAM2

For dissolve:

    T - duration/2
        CAM1

    T
        50/50

    T + duration/2
        CAM2

The Program monitor should preview the actual transition.

This is important because otherwise the operator cannot judge the finished edit.

---

# 38. Phase 36 — "Live Directing" Mode

Create a dedicated mode where almost everything except the
multiview is hidden.

Possible UI:

    ┌───────────────────────────────────────────┐
    │                                           │
    │                 PROGRAM                   │
    │                                           │
    └───────────────────────────────────────────┘

    ┌──────┬──────┬──────┬──────┐
    │ C1   │ C2   │ C3   │ C4   │
    ├──────┼──────┼──────┼──────┤
    │ C5   │ C6   │ C7   │ C8   │
    └──────┴──────┴──────┴──────┘

    SHOW 01:13:42.240     PLAYING 1.0x

    [CUT] [DISSOLVE] [UNDO]

This should be the primary theatre workflow.

---

# 39. Phase 37 — Directing Session

Add:

    Start Directing

This should:

1. Save the project.
2. Reset playback to the desired start point.
3. Clear or preserve the existing decision list.
4. Enable keyboard controls.
5. Start playback.

At the end:

    Stop Directing

automatically saves.

This reduces the possibility of losing an hour of directing.

---

# 40. Phase 38 — Autosave

Very important.

Autosave:

    every 10–30 seconds

and after every decision.

Use atomic writes:

    project.tmp
         ↓
    fsync if appropriate
         ↓
    rename
         ↓
    project.ovm

Keep a small number of backup versions.

Example:

    project.ovm
    project.ovm.bak1
    project.ovm.bak2

---

# 41. Phase 39 — Crash Recovery

On startup:

    "A recovered project is available."

Because the decision list is tiny compared with the media, saving it
frequently should be cheap.

---

# 42. Phase 40 — Frame Accuracy

Once the basic system works, explicitly define what "frame accurate" means.

Internally use:

    integer frame numbers

where practical.

For example:

    25 fps

    frame 0
    frame 1
    frame 2
    ...

Avoid floating-point seconds as the canonical representation of edit
positions.

Use a rational frame/time representation where possible.

For example:

    RationalTime {
        value: i64,
        timescale: i32,
    }

or a project-level framerate:

    25 fps

and store decision positions as frame numbers.

This will prevent subtle cumulative timing errors.

---

# 43. Phase 41 — Variable Frame Rate

Do not support VFR in the first MVP unless necessary.

Detect it during media inspection.

Display:

    ⚠ Variable frame rate

Initially recommend transcoding to CFR.

Later add proper timestamp-based handling.

---

# 44. Phase 42 — Mixed Framerates

Eventually allow:

    CAM1 25 fps
    CAM2 50 fps
    CAM3 25 fps

The show timeline should remain independent of source frame rate.

The renderer handles conversion.

---

# 45. Phase 43 — Camera Availability

A camera can be unavailable.

For:

    source_time < 0

display:

    NO SIGNAL

rather than attempting to decode.

Likewise after the source ends.

This is particularly useful for theatre recordings where cameras may
start/stop independently.

---

# 46. Phase 44 — Camera Naming

Allow:

    CAM1 → Wide
    CAM2 → Stage Left
    CAM3 → Stage Right
    CAM4 → Close-up
    CAM5 → Audience

Display the name prominently in multiview.

Keyboard shortcuts should remain numeric.

---

# 47. Phase 45 — Safe Directing

Add an optional confirmation indicator:

    PROGRAM
    ● CAM3

    PREVIEW
    ○ CAM5

Never allow accidental camera selection to immediately change Program.

Camera selection changes Preview.

Only TAKE changes Program.

This is fundamental to the vision-mixer workflow.

---

# 48. Phase 46 — Dedicated Hardware Controllers

Later support MIDI/HID devices.

Possible inputs:

- Stream Deck
- MIDI controller
- X-Keys
- generic keyboard
- USB buttons

Internally:

    InputEvent::Take(CameraId)
    InputEvent::Select(CameraId)
    InputEvent::Play
    InputEvent::Pause

This makes hardware support independent of the switcher.

---

# 49. Phase 47 — Markers

Add non-editing markers:

    "actor enters"
    "scene change"
    "lighting change"
    "intermission"
    "bad camera"

These are annotations, not edit decisions.

Example:

    struct Marker {
        time: FrameNumber,
        label: String,
    }

This will become extremely useful for long theatre productions.

---

# 50. Phase 48 — Scene/Chapter Support

Optional.

Allow markers to become chapters:

    Scene 1
    Scene 2
    Scene 3

This can make navigating a 2–3 hour performance much easier.

---

# 51. Phase 49 — Multi-monitor Support

Eventually support:

Monitor 1:

    Multiview + Program

Monitor 2:

    Fullscreen Program

Monitor 3:

    Timeline / controls

For egui, this may require careful handling of multiple native windows,
depending on the chosen eframe architecture.

Do not make this an MVP requirement.

---

# 52. Phase 50 — Audio Monitoring

Eventually:

    Program audio
    Master audio
    Camera audio

Add:

    mute
    solo
    volume

But avoid building a mixer.

The first useful version only needs:

    master audio → output

---

# 53. Phase 51 — Final Render Verification

Before declaring a render successful:

- verify output exists
- verify duration
- verify frame count
- verify audio duration
- verify exit status
- capture FFmpeg stderr
- show render progress

Example:

    Rendering...

    73.4%

    CAM3 → 00:31:24
    ETA: 04:32

---

# 54. Phase 52 — Render Queue

Eventually support:

    Render Queue

    ┌───────────────────────────────┐
    │ Theatre Master     4K ProRes  │
    │ Theatre Preview   1080p H264  │
    │ Review Copy       720p H264   │
    └───────────────────────────────┘

This is useful because the actual render can be slow while the
directing application remains lightweight.

---

# 55. Phase 53 — Testing Strategy

Testing should focus heavily on the timeline mathematics.

Create unit tests for:

    source_time(show_time, offset)

Examples:

    show=10, offset=0 -> source=10
    show=10, offset=5 -> source=5
    show=10, offset=15 -> source=-5

Test decision evaluation:

    decisions:
      0 CAM1
      10 CAM2
      20 CAM3

    0  -> CAM1
    9  -> CAM1
    10 -> CAM2
    19 -> CAM2
    20 -> CAM3

Test transitions.

Test camera endings.

Test camera starting halfway through.

Test seeking.

Test save/load.

Test render-plan generation.

---

# 56. Phase 54 — Golden Test Projects

Create small test media.

For example:

    camera1 = solid red / timestamp
    camera2 = solid green / timestamp
    camera3 = solid blue / timestamp
    camera4 = checkerboard / timestamp

Every frame can contain:

    CAM 2
    SOURCE 00:01:23.400

This makes synchronization and rendering bugs extremely obvious.

A 10-second synthetic test project can validate the entire pipeline.

---

# 57. Phase 55 — Integration Tests

Automate:

    project
       ↓
    render
       ↓
    inspect output
       ↓
    verify expected frame/source

Example:

    0–5 sec     CAM1
    5–10 sec    CAM2

Expected output:

    frames 0–124    CAM1
    frames 125–249  CAM2

These tests will protect the renderer as it becomes more sophisticated.

---

# 58. Phase 56 — Performance Benchmarks

Create benchmarks for:

- 4 × 1080p playback
- 8 × 1080p playback
- 4 × 4K playback
- 8 × 4K playback

Measure:

    CPU
    GPU
    memory
    decode latency
    frame drops
    seek latency

Do not optimize before measuring.

---

# 59. Phase 57 — MVP Definition

The application should be considered MVP-complete when it can:

### Media

- [x] Import 4–8 video files
- [x] Inspect metadata
- [x] Manually specify offsets
- [x] Save project

### Playback

- [x] Common show clock
- [x] Synchronized playback
- [x] Play/pause
- [x] Seek
- [x] Frame step
- [x] Playback speed

### Multiview

- [x] 4–8 camera views
- [x] Camera labels
- [x] Preview indicator
- [x] Program indicator

### Switching

- [x] Keyboard camera selection
- [x] CUT
- [x] Preview/Program
- [x] Record decisions
- [x] Undo

### Editing

- [x] Decision timeline
- [x] Modify decisions
- [x] Delete decisions
- [x] Replay finished program

### Rendering

- [x] Generate render plan
- [x] Render cuts
- [x] Render dissolves
- [x] Output H.264
- [x] External/master audio

At this point the program is genuinely useful.

---

# 60. Post-MVP Features

Only after the MVP is stable:

- Proxy generation
- FCPXML
- CMX3600 EDL
- ProRes
- DNxHR
- hardware decoding
- hardware controllers
- MIDI
- Stream Deck
- multiple monitors
- markers
- scene/chapter support
- render queue
- crash recovery
- media relinking
- audio monitoring
- variable framerate
- mixed framerate
- multi-track audio
- transition library
- dip-to-black
- wipes
- picture-in-picture
- graphics
- live inputs

Most of these should remain optional.

---

# 61. Suggested Rust Domain Model

A possible initial model:

    type Frame = i64;

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    struct CameraId(u32);

    struct Project {
        version: u32,
        name: String,
        frame_rate: FrameRate,
        duration: Frame,
        cameras: Vec<Camera>,
        audio: Option<AudioSource>,
        decisions: Vec<Decision>,
        markers: Vec<Marker>,
    }

    struct Camera {
        id: CameraId,
        name: String,
        path: PathBuf,
        offset: Frame,
        enabled: bool,
        media_info: Option<MediaInfo>,
        proxy_path: Option<PathBuf>,
    }

    struct Decision {
        at: Frame,
        camera: CameraId,
        transition: Transition,
    }

    enum Transition {
        Cut,
        Dissolve {
            duration: Frame,
        },
    }

    struct AudioSource {
        path: PathBuf,
        offset: Frame,
    }

The exact representation can change.

The important thing is that:

**Project state must not contain rendered video.**

---

# 62. Recommended State Separation

Keep these separate:

    Project
       │
       ├── persistent data
       │
       └── decisions

    PlaybackState
       │
       ├── current position
       ├── playing
       └── rate

    SwitcherState
       │
       ├── preview
       ├── program
       └── transition

    MediaRuntime
       │
       ├── decoders
       ├── frame queues
       └── textures

    RenderState
       │
       ├── progress
       ├── current segment
       └── process state

This will prevent the application from turning into one enormous
`App` struct.

---

# 63. Development Order

The recommended implementation order is:

    1.  egui application skeleton
    2.  Project model
    3.  Save/load
    4.  Media probing
    5.  One-video playback
    6.  Show clock
    7.  Multiple synchronized videos
    8.  Multiview
    9.  Program monitor
    10. Switcher state
    11. Decision recording
    12. Decision evaluation
    13. Program replay
    14. Decision timeline
    15. Edit/undo/redo
    16. Render-plan generation
    17. FFmpeg cut rendering
    18. Dissolve rendering
    19. Master audio
    20. FCPXML
    21. Proxies
    22. Performance optimization
    23. Hardware controls
    24. Polish

Do not reverse this order.

In particular:

**Do not start with the renderer.**

The unique value of the application is the directing workflow.

---

# 64. First Prototype

The very first useful prototype should be ridiculously small.

It should have:

    ┌─────────────────────────────────────┐
    │             PROGRAM                 │
    │                                     │
    │                                     │
    └─────────────────────────────────────┘

    ┌────────┬────────┬────────┬────────┐
    │ CAM 1  │ CAM 2  │ CAM 3  │ CAM 4  │
    └────────┴────────┴────────┴────────┘

    00:12:31.240

    [ PLAY ] [ PAUSE ]

    Preview: CAM 3
    Program: CAM 1

    Decisions:
      00:00:00 CAM1
      00:00:12 CAM3

And the only keyboard commands:

    1 = CAM1
    2 = CAM2
    3 = CAM3
    4 = CAM4
    SPACE = TAKE
    P = PLAY/PAUSE

If this works reliably, the core idea is proven.

Everything else can grow around it.

---

# 65. Project Philosophy

Keep asking:

> "Does this help someone sit down and direct a prerecorded performance?"

If not, it probably doesn't belong in the core application.

The ideal workflow should eventually be:

    Import footage
          ↓
    Set offsets
          ↓
    Press PLAY
          ↓
    Direct the show
          ↓
    Review
          ↓
    Correct a few cuts
          ↓
    RENDER

No giant timeline setup.

No complicated media bins.

No keyframe editing.

No live streaming.

No unnecessary broadcast infrastructure.

Just:

**watch the performance and call the shots.**

---

# 66. Possible Project Names

Temporary working names:

- OfflineVision
- ReVision
- PostVision
- Virtual Vision Mixer
- OVM — Offline Vision Mixer
- StageMix
- Rehearsal
- Director
- CutDesk
- VisionDesk
- PostSwitcher
- ShowSwitcher

A name can wait.

The architecture matters more.

---

# 67. Licensing

If the intention is to release the application as FOSS:

    GPL-3.0-or-later

would be a natural choice for a desktop application containing
GPL-compatible media infrastructure.

Alternatively:

    MIT
    Apache-2.0

if maximum reuse is more important.

Before choosing, check the licenses of whichever FFmpeg/GStreamer/
other native libraries are ultimately distributed with the application.

Do not choose the project's license solely based on the Rust crates.

---

# 68. Final Target Architecture

The finished application should conceptually look like this:

                         PROJECT
                            │
                  ┌─────────┴─────────┐
                  │                   │
               CAMERAS              AUDIO
                  │                   │
                  ▼                   ▼
              OFFSETS              OFFSET
                  │                   │
                  └─────────┬─────────┘
                            │
                       SHOW CLOCK
                            │
             ┌──────────────┼──────────────┐
             │              │              │
             ▼              ▼              ▼
          CAMERA 1       CAMERA 2        CAMERA N
             │              │              │
             └──────────────┼──────────────┘
                            │
                         MULTIVIEW
                            │
                            ▼
                      VIRTUAL MIXER
                            │
                       ┌────┴────┐
                       │         │
                    PREVIEW    PROGRAM
                       │         │
                       └────┬────┘
                            │
                       TAKE EVENTS
                            │
                            ▼
                      DECISION LIST
                            │
                  ┌─────────┴──────────┐
                  │                    │
               REPLAY               RENDER
                  │                    │
                  ▼                    ▼
              PROGRAM             FFmpeg
                                     │
                                     ▼
                                FINAL MASTER

The key architectural principle is:

**The switcher is an input device for creating an edit decision list,
not a video renderer.**

That single decision is what keeps the application small, deterministic,
and purpose-built for prerecorded theatre multicam production.
