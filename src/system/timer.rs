pub struct Timer {
    pub duration: f32,
    pub current_time: f32,
}

impl Timer {
    pub fn new(duration: f32) -> Self {
        Timer {
            duration,
            current_time: 0.0
        }
    }
    
    pub fn track(&mut self, dt: f32) {

        if self.current_time >= self.duration {
            self.current_time = self.duration;
            return;
        }
        
        self.current_time += dt;
    }
    
    pub fn is_started(&self) -> bool {
        return  self.current_time > 0.0;
    }
       
    pub fn is_done(&self) -> bool {
        return self.current_time >= self.duration;
    }
    
    pub fn reset(&mut self) {
        self.current_time = 0.0;
    }

    pub fn progress(&self) -> f32 {
        return (self.current_time / self.duration).clamp(0.0, 1.0);
    }  
}