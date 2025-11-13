/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-swervelib                     */
/*    File: swervemotor.rs                          */
/*    Author: Andrew Bobay                          */
/*    Date Created: Nov 9th 2025 11:30AM            */
/*    Date Modified: Nov 12th 2025 2:30PM           */
/*    Team: BBR1                                    */
/*    Description: swervelib modules                */
/*                                                  */
/* ------------------------------------------------ */

use alloc::vec::Vec;
use vexide::prelude::*;
use crate::eclipselib; 

pub struct SwerveMotor{
    motor: eclipselib::motors::AdvMotor
}

impl SwerveMotor{
    pub fn new(port: SmartPort, gearset: Gearset, direction: Direction) -> Self{
        Self{motor: eclipselib::motors::AdvMotor::new(port, gearset, direction)}
    }
}

// Used for 88W implemenation
pub struct SwerveMotorGroup{
    motors: Vec<SwerveMotor>
}
impl SwerveMotorGroup{
    pub fn new(motors: Vec<SwerveMotor>) -> Self{
        Self{motors}
    }
}


