/// Trait definition to abstract volumetric pumping control across pump types
pub trait VolumetricPump {
    /// pump a specific volume
    fn pump_ml(ml: usize);
}
