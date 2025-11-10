/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-swervelib                     */
/*    File: swervemotor.rs                          */
/*    Author: Andrew Bobay                          */
/*    Date Created: Nov 9th 2025 11:30AM            */
/*    Date Modified: Nov 9th 2025 11:30PM           */
/*    Team: BBR1                                    */
/*    Description: swervelib modules                */
/*                                                  */
/* ------------------------------------------------ */

use vexide::prelude::*;

pub struct SwerveMotor{
    motor: Motor
}

impl SwerveMotor{
pub fn new(port: SmartPort, gearset: Gearset, direction: Direction) -> Self{
        Self{motor: Motor::new(port, gearset, direction)}
}


}