use zkml_robotics::{load_model, execute_command, verify_proof, RobotCommand};
use std::time::Instant;
use anyhow::Result;

fn main() -> Result<()> {
    load_model("models/robot_model.onnx")?;

    let commands = vec![
        RobotCommand { action: "move_forward".into(), params: vec![1.0] },
        RobotCommand { action: "turn_left".into(), params: vec![90.0] },
        RobotCommand { action: "move_forward".into(), params: vec![2.0] },
    ];

    for cmd in commands {
        let start = Instant::now();
        let proof = execute_command(&cmd)?;
        let duration = start.elapsed();

        let valid = verify_proof(&proof)?;
        println!(
            "Command: {:?} | Proof valid: {} | Time elapsed: {:?}",
            cmd.action, valid, duration
        );
    }

    Ok(())
}
