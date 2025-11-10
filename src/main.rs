/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-eclipselib                    */
/*    File: main.rs                                 */
/*    Author: Andrew Bobay                          */
/*    Date Created: Sep 23rd 2025 12:00PM           */
/*    Date Modified: Nov 05th 2025 02:42PM          */
/*    Team: BBR1                                    */
/*    Description: Example Main file                */
/*                                                  */
/* ------------------------------------------------ */

#![no_main]
#![no_std]

use autons::{
    prelude::*,
    simple::{route, SimpleSelect},
};
use vexide::{devices::peripherals, prelude::*};
mod swervelib;
mod eclipselib;
extern crate alloc;
pub use alloc::vec;

struct Robot { 
    controller: Controller,
    swerve: swervelib::swervedrive::SwerveDrive,
    smartmtr: eclipselib::motors::AdvMotor

}

impl Robot{
    async fn match_auto(&mut self) {

    }
    async fn elims_auto(&mut self) {

    }
    async fn skills_auto(&mut self) {

    }
}

impl SelectCompete for Robot {
    async fn connected(&mut self) {
        println!("Pre-Initiated");

    }
    async fn disconnected(&mut self) {
        println!("Controller Disconnected")
    }
    async fn disabled(&mut self) {
        println!("Awaiting Competition")
    }
    async fn driver(&mut self) {
        println!("Driver!");

        loop{
            let controller_state = self.controller.state().unwrap_or_default();

        }
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {

    let robot = Robot {
        controller: peripherals.primary_controller,
        swerve: swervelib::swervedrive::SwerveDrive::new(
            swervelib::swervemod::SwerveModule::new(
                swervelib::swervemotor::SwerveMotor::new(peripherals.port_1, Gearset::Blue, Direction::Forward), 
                swervelib::swervemotor::SwerveMotor::new(peripherals.port_2, Gearset::Blue, Direction::Forward), 
                RotationSensor::new(peripherals.port_3, Direction::Forward),
            ),
            swervelib::swervemod::SwerveModule::new(
                swervelib::swervemotor::SwerveMotor::new(peripherals.port_4, Gearset::Blue, Direction::Forward), 
                swervelib::swervemotor::SwerveMotor::new(peripherals.port_5, Gearset::Blue, Direction::Forward), 
                RotationSensor::new(peripherals.port_6, Direction::Forward),
            ),
            InertialSensor::new(peripherals.port_7),
        ),
        smartmtr: eclipselib::motors::AdvMotor::new(
            peripherals.port_8, Gearset::Blue, Direction::Forward,
        )

    };

    robot
        .compete(SimpleSelect::new(
            peripherals.display,
            [
                route!("Match Auto", Robot::match_auto),
                route!("Elims Auto", Robot::elims_auto),
                route!("Skills", Robot::skills_auto),
            ],
        ))
        .await;
}
