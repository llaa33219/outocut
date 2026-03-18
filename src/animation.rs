use crate::models::*;
use serde_json::Value;

pub struct Animator;

impl Animator {
    pub fn evaluate_f64(property: &AnimatedProperty<f64>, time: f64) -> f64 {
        if let Some(keyframes) = &property.keyframes {
            if keyframes.is_empty() {
                return property.value;
            }

            let sortedkeyframes = Self::sort_keyframes(keyframes);

            if time <= sortedkeyframes[0].time {
                return Self::json_to_f64(&sortedkeyframes[0].value);
            }
            if time >= sortedkeyframes.last().map(|k| k.time).unwrap_or(0.0) {
                return Self::json_to_f64(&sortedkeyframes.last().unwrap().value);
            }

            for i in 0..sortedkeyframes.len() - 1 {
                let k1 = &sortedkeyframes[i];
                let k2 = &sortedkeyframes[i + 1];

                if time >= k1.time && time <= k2.time {
                    let progress = (time - k1.time) / (k2.time - k1.time);
                    let eased_progress = Self::apply_easing(progress, &k1.easing);
                    return Self::interpolate_f64(&k1.value, &k2.value, eased_progress);
                }
            }
        }

        property.value
    }

    pub fn evaluate_vec_f64(property: &AnimatedProperty<Vec<f64>>, time: f64) -> Vec<f64> {
        if let Some(keyframes) = &property.keyframes {
            if keyframes.is_empty() {
                return property.value.clone();
            }

            let sortedkeyframes = Self::sort_keyframes(keyframes);

            if time <= sortedkeyframes[0].time {
                return Self::json_to_vec_f64(&sortedkeyframes[0].value);
            }
            if time >= sortedkeyframes.last().map(|k| k.time).unwrap_or(0.0) {
                return Self::json_to_vec_f64(&sortedkeyframes.last().unwrap().value);
            }

            for i in 0..sortedkeyframes.len() - 1 {
                let k1 = &sortedkeyframes[i];
                let k2 = &sortedkeyframes[i + 1];

                if time >= k1.time && time <= k2.time {
                    let progress = (time - k1.time) / (k2.time - k1.time);
                    let eased_progress = Self::apply_easing(progress, &k1.easing);
                    return Self::interpolate_vec_f64(&k1.value, &k2.value, eased_progress);
                }
            }
        }

        property.value.clone()
    }

    fn sort_keyframes(keyframes: &[Keyframe]) -> Vec<&Keyframe> {
        let mut sorted: Vec<&Keyframe> = keyframes.iter().collect();
        sorted.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        sorted
    }

    fn apply_easing(t: f64, easing: &Option<Easing>) -> f64 {
        match easing {
            None | Some(Easing::Linear) => t,
            Some(Easing::EaseIn) => t * t,
            Some(Easing::EaseOut) => 1.0 - (1.0 - t) * (1.0 - t),
            Some(Easing::EaseInOut) => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Some(Easing::EaseInCubic) => t * t * t,
            Some(Easing::EaseOutCubic) => 1.0 - (1.0 - t).powi(3),
            Some(Easing::EaseInOutCubic) => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Some(Easing::EaseInBack) => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            Some(Easing::EaseOutBack) => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
            Some(Easing::EaseInOutBack) => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if t < 0.5 {
                    ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
                }
            }
            Some(Easing::EaseInElastic) => {
                if t == 0.0 || t == 1.0 {
                    return t;
                }
                let c4 = (2.0 * std::f64::consts::PI) / 3.0;
                -(2.0_f64.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * c4).sin()).abs()
            }
            Some(Easing::EaseOutElastic) => {
                if t == 0.0 || t == 1.0 {
                    return t;
                }
                let c4 = (2.0 * std::f64::consts::PI) / 3.0;
                2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
            }
            Some(Easing::EaseInOutElastic) => {
                if t == 0.0 || t == 1.0 {
                    return t;
                }
                let c5 = (2.0 * std::f64::consts::PI) / 4.5;
                if t < 0.5 {
                    -(2.0_f64.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0
                } else {
                    (2.0_f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0 + 1.0
                }
            }
            Some(Easing::EaseInBounce) => 1.0 - Self::ease_out_bounce(1.0 - t),
            Some(Easing::EaseOutBounce) => Self::ease_out_bounce(t),
            Some(Easing::EaseInOutBounce) => {
                if t < 0.5 {
                    (1.0 - Self::ease_out_bounce(1.0 - 2.0 * t)) / 2.0
                } else {
                    (1.0 + Self::ease_out_bounce(2.0 * t - 1.0)) / 2.0
                }
            }
            Some(Easing::CubicBezier(x1, y1, x2, y2)) => Self::cubic_bezier(*x1, *y1, *x2, *y2, t),
        }
    }

    fn ease_out_bounce(t: f64) -> f64 {
        let n1 = 7.5625;
        let d1 = 2.75;

        if t < 1.0 / d1 {
            n1 * t * t
        } else if t < 2.0 / d1 {
            let t = t - 1.5 / d1;
            n1 * t * t + 0.75
        } else if t < 2.5 / d1 {
            let t = t - 2.25 / d1;
            n1 * t * t + 0.9375
        } else {
            let t = t - 2.625 / d1;
            n1 * t * t + 0.984375
        }
    }

    fn cubic_bezier(x1: f64, y1: f64, x2: f64, y2: f64, t: f64) -> f64 {
        let cx = 3.0 * x1;
        let bx = 3.0 * (x2 - x1) - cx;
        let ax = 1.0 - cx - bx;

        let cy = 3.0 * y1;
        let by = 3.0 * (y2 - y1) - cy;
        let ay = 1.0 - cy - by;

        let x = ((ax * t + bx) * t + cx) * t;
        let y = ((ay * t + by) * t + cy) * t;

        Self::solve_cubic_bezier(y1, y2, y, t)
    }

    fn solve_cubic_bezier(y1: f64, y2: f64, y: f64, t_guess: f64) -> f64 {
        let mut t = t_guess;
        for _ in 0..10 {
            let cx = 3.0 * y1;
            let bx = 3.0 * (y2 - y1) - cx;
            let ax = 1.0 - cx - bx;
            let y_guess = ((ax * t + bx) * t + cx) * t;
            if (y_guess - y).abs() < 0.001 {
                break;
            }
            let cx2 = 3.0 * y1;
            let bx2 = 3.0 * (y2 - y1) - cx2;
            let ax2 = 1.0 - cx2 - bx2;
            let slope =
                (((ax2 * (t + 0.001) + bx2) * (t + 0.001) + cx2) * (t + 0.001) - y_guess) / 0.001;
            t = (t - (y_guess - y) / slope).clamp(0.0, 1.0);
        }
        t
    }

    fn json_to_f64(value: &Value) -> f64 {
        match value {
            Value::Number(n) => n.as_f64().unwrap_or(0.0),
            Value::Array(arr) => {
                if let Some(first) = arr.first() {
                    Self::json_to_f64(first)
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }

    fn json_to_vec_f64(value: &Value) -> Vec<f64> {
        match value {
            Value::Array(arr) => arr.iter().map(|v| Self::json_to_f64(v)).collect(),
            _ => vec![],
        }
    }

    fn interpolate_f64(value1: &Value, value2: &Value, t: f64) -> f64 {
        let v1 = Self::json_to_f64(value1);
        let v2 = Self::json_to_f64(value2);
        v1 + (v2 - v1) * t
    }

    fn interpolate_vec_f64(value1: &Value, value2: &Value, t: f64) -> Vec<f64> {
        let v1 = Self::json_to_vec_f64(value1);
        let v2 = Self::json_to_vec_f64(value2);

        v1.iter()
            .zip(v2.iter())
            .map(|(a, b)| a + (b - a) * t)
            .collect()
    }

    pub fn evaluate_transform(transform: &Transform, time: f64) -> ComputedTransform {
        ComputedTransform {
            anchor: Self::evaluate_vec_f64(&transform.anchor, time),
            position: Self::evaluate_vec_f64(&transform.position, time),
            scale: Self::evaluate_vec_f64(&transform.scale, time),
            rotation: Self::evaluate_f64(&transform.rotation, time),
            skew: Self::evaluate_vec_f64(&transform.skew, time),
            skew_axis: Self::evaluate_f64(&transform.skewAxis, time),
        }
    }

    pub fn is_layer_active(layer: &Layer, time: f64) -> bool {
        let layer_start = layer.startTime;
        let layer_end = layer.startTime + layer.duration;
        let in_point = layer.inPoint.unwrap_or(0.0);
        let out_point = layer.outPoint.unwrap_or(layer.duration);

        time >= layer_start
            && time < layer_end
            && time >= (layer_start + in_point)
            && time < (layer_start + out_point)
    }
}

#[derive(Debug, Clone)]
pub struct ComputedTransform {
    pub anchor: Vec<f64>,
    pub position: Vec<f64>,
    pub scale: Vec<f64>,
    pub rotation: f64,
    pub skew: Vec<f64>,
    pub skew_axis: f64,
}
