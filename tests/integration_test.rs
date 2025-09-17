use zkml_robotics::{load_model, execute_command, verify_proof, RobotCommand};
use anyhow::Result;

#[test]
fn test_move_forward() -> Result<()> {
    load_model("models/robot_model.onnx")?;

    let command = RobotCommand {
        action: "move_forward".into(),
        params: vec![1.0],
    };

    let proof = execute_command(&command)?;
    assert!(verify_proof(&proof)?);

    Ok(())
}

#[test]
fn test_turn_left() -> Result<()> {
    load_model("models/robot_model.onnx")?;

    let command = RobotCommand {
        action: "turn_left".into(),
        params: vec![90.0],
    };

    let proof = execute_command(&command)?;
    assert!(verify_proof(&proof)?);

    Ok(())
}

#[test]
fn test_complex_path() -> Result<()> {
    load_model("models/robot_model.onnx")?;

    let commands = vec![
        RobotCommand { action: "move_forward".into(), params: vec![1.0] },
        RobotCommand { action: "turn_right".into(), params: vec![90.0] },
        RobotCommand { action: "move_forward".into(), params: vec![2.0] },
    ];

    for cmd in commands {
        let proof = execute_command(&cmd)?;
        assert!(verify_proof(&proof)?);
    }

    Ok(())
}
