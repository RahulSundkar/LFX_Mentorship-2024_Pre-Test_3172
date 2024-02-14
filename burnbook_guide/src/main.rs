use std::io;
use burn::backend::Autodiff;
use burn::backend::Wgpu;
use burn::backend::wgpu::{AutoGraphicsApi, WgpuDevice};

use burn::data::dataset::Dataset;
use burn::optim::AdamConfig;

use burnbook_guide::model::ModelConfig;
use burnbook_guide::training;
use burnbook_guide::inference;

fn main() {
    type MyBackend = Wgpu<AutoGraphicsApi, f32, i32>;
    type MyAutodiffBackend = Autodiff<MyBackend>;

    let device = WgpuDevice::default();
    let artifact_dir = "artifacts";

    println!("Enter 0 to train\nEnter any other number to run inference\nYour Choice: ");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Failed to read line!");

    let choice: u8 = choice.trim().parse().expect("A number was expected!");

    if choice == 0 {
        training::train::<MyAutodiffBackend>(
            artifact_dir,
            training::TrainingConfig::new(ModelConfig::new(10, 512), AdamConfig::new()),
            device.clone(),
        );
    }
    else {
        inference::infer::<MyBackend> (
            artifact_dir,
            device,
            burn::data::dataset::vision::MNISTDataset::test().get(55).unwrap(),
        );
    }
}
