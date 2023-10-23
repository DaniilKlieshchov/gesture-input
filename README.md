# GestureInput: Bind Touchpad Gestures to Bash Commands

GestureInput is a capstone project for the Ukrainian Rust Programming Bootcamp. It allows users to bind bash commands to specific touchpad gestures using a configuration file. This application uses the `libinput` library to detect and process touchpad gestures.

## Installation

Before using GestureInput, you must install the required `libinput` dependencies:

```bash
sudo apt-get install libinput-dev
```

## Supported Gestures

GestureInput currently supports the following touchpad gestures:

- `ThreeFingerSwipeGradual`
- `ThreeFingerSwipe`
- `FourFingerSwipe`
- `Pinch`
- `Spread`
- `Hold`

## Configuration

To bind gestures to bash commands, create a `config.yaml` file. For each gesture, specify its type and the corresponding action. 

Here's a sample `config.yaml`:

```yaml
gestures: 
  - !ThreeFingerSwipeGradual
      direction: Down
      action: 'amixer sset Master 1%-'

  - !ThreeFingerSwipeGradual
      direction: Up
      action: 'amixer sset Master 1%+'   

  - !Pinch
      action: echo 'pinched'

  - !Spread
      action: echo 'spreaded'

  - !FourFingerSwipe
      direction: Down
      action: echo 'swiped down with 4 fingers'

  - !FourFingerSwipe
      direction: Up
      action: echo 'swiped up with 4 fingers'

  - !FourFingerSwipe
      direction: Left
      action: echo 'swiped left with 4 fingers'

  - !FourFingerSwipe
      direction: Right
      action: echo 'swiped right with 4 fingers'

  - !Hold
      duration: 500
      action: "echo 'held for 500 milliseconds'"

  - !Hold
      duration: 5000
      action: "echo 'held for 5000 milliseconds (5 seconds)'"
```

**Note**: All actions execute a bash command only after finishing the gesture, i.e., when you remove your finger from the touchpad. The exception is for gradual actions, which will execute your command during the movement of your fingers.
