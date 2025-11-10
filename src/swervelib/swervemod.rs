/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-swervelib                     */
/*    File: swervemod.rs                            */
/*    Author: Andrew Bobay                          */
/*    Date Created: Nov 9th 2025 11:30AM            */
/*    Date Modified: Nov 9th 2025 11:30PM           */
/*    Team: BBR1                                    */
/*    Description: swervelib modules                 */
/*                                                  */
/* ------------------------------------------------ */

use vexide::prelude::*;
use vexide::devices::PortError; 
use crate::swervelib::swervemotor;

pub struct SwerveModule{
    smartmtr_top: swervemotor::SwerveMotor,
    smartmtr_bottom: swervemotor::SwerveMotor,
    azimuth: RotationSensor
}
impl SwerveModule{
    pub fn new(smartmtr_top:swervemotor::SwerveMotor, smartmtr_bottom:swervemotor::SwerveMotor, azimuth:RotationSensor)->Self{
        Self{smartmtr_top, smartmtr_bottom, azimuth}
    }

    pub fn get_azimuth(&mut self)->Result<Position, PortError>{
        self.azimuth.position()
    }
}
