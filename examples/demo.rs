use zkml_robotics::{load_model, execute_command, verify_proof, RobotCommand};
use anyhow::Result;

fn main() -> Result<()> {
    // Load AI model
    load_model("models/robot_model.onnx")?;

    // Command robot maju 1 meter
    let command = RobotCommand {
        action: "move_forward".into(),
        params: vec![1.0],
    };

    // Jalankan command dan generate proof
    let proof = execute_command(&command)?;
    println!("Proof generated: {}", proof);

    // Verifikasi proof
    let valid = verify_proof(&proof)?;
    println!("Proof valid: {}", valid);

    Ok(())
}
