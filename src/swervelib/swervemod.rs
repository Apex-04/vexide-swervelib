/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-swervelib                     */
/*    File: swervemod.rs                            */
/*    Author: Andrew Bobay                          */
/*    Date Created: Nov 9th 2025 11:30AM            */
/*    Date Modified: Nov 12th 2025 2:30PM           */
/*    Team: BBR1                                    */
/*    Description: swervelib modules                 */
/*                                                  */
/* ------------------------------------------------ */

use vexide::prelude::*;
use vexide::devices::PortError; 
use crate::swervelib;
pub struct SwerveModule {
    smartmtr_top1: swervelib::swervemotors::SwerveMotor,
    smartmtr_bottom1: swervelib::swervemotors::SwerveMotor,
    smartmtr_top2: Option<swervelib::swervemotors::SwerveMotor>,
    smartmtr_bottom2: Option<swervelib::swervemotors::SwerveMotor>,
    azimuth: RotationSensor,
}
#[allow(unused)]
impl SwerveModule {
    /// Create a standard 2-motor swerve module
    pub fn new(
        smartmtr_top1: swervelib::swervemotors::SwerveMotor,
        smartmtr_bottom1: swervelib::swervemotors::SwerveMotor,
        azimuth: RotationSensor,
    ) -> Self {
        Self {
            smartmtr_top1,
            smartmtr_bottom1,
            smartmtr_top2: None,
            smartmtr_bottom2: None,
            azimuth,
        }
    }

    /// Create an 88w swerve module
    pub fn new88w(
        smartmtr_top1: swervelib::swervemotors::SwerveMotor,
        smartmtr_top2: swervelib::swervemotors::SwerveMotor,
        smartmtr_bottom1: swervelib::swervemotors::SwerveMotor,
        smartmtr_bottom2: swervelib::swervemotors::SwerveMotor,
        azimuth: RotationSensor,
    ) -> Self {
        Self {
            smartmtr_top1,
            smartmtr_bottom1,
            smartmtr_top2: Some(smartmtr_top2),
            smartmtr_bottom2: Some(smartmtr_bottom2),
            azimuth,
        }
    }

    pub fn get_azimuth(&mut self)->Result<Position, PortError>{
        self.azimuth.position()
    }
}
