use eframe::egui;
use egui::{Color32, Pos2};
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    position: Pos2,
    visited: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Edge {
    from: usize,
    to: usize,
    weight: i32,
    selected: bool,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct PrimVisualizer {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    priority_queue: BinaryHeap<Edge>,
    visited: HashSet<usize>,
    current_edge: Option<Edge>,
    running: bool,
    step_delay: f32,
    accumulated_time: f32,
}

impl Default for PrimVisualizer {
    fn default() -> Self {
        let mut nodes = Vec::new();
        
        for i in 0..5 {
            let angle = (i as f32) * std::f32::consts::TAU / 5.0;
            nodes.push(Node {
                id: i,
                position: Pos2::new(
                    300.0 + 200.0 * angle.cos(),
                    300.0 + 200.0 * angle.sin(),
                ),
                visited: false,
            });
        }

        let edges = vec![
            Edge { from: 0, to: 1, weight: 2, selected: false },
            Edge { from: 0, to: 3, weight: 6, selected: false },
            Edge { from: 1, to: 2, weight: 3, selected: false },
            Edge { from: 1, to: 3, weight: 8, selected: false },
            Edge { from: 1, to: 4, weight: 5, selected: false },
            Edge { from: 2, to: 4, weight: 7, selected: false },
            Edge { from: 3, to: 4, weight: 9, selected: false },
        ];

        let mut vis = Self {
            nodes,
            edges,
            priority_queue: BinaryHeap::new(),
            visited: HashSet::new(),
            current_edge: None,
            running: false,
            step_delay: 0.5,
            accumulated_time: 0.0,
        };

        vis.initialize_algorithm();
        vis
    }
}

impl PrimVisualizer {
    fn initialize_algorithm(&mut self) {
        self.visited.clear();
        self.priority_queue.clear();
        self.current_edge = None;
        self.edges.iter_mut().for_each(|e| e.selected = false);
        self.nodes.iter_mut().for_each(|n| n.visited = false);

        if !self.nodes.is_empty() {
            self.visited.insert(0);
            self.nodes[0].visited = true;
            self.add_edges_to_queue(0);
        }
    }

    fn add_edges_to_queue(&mut self, node_id: usize) {
        for edge in &self.edges {
            if edge.from == node_id && !self.visited.contains(&edge.to) {
                self.priority_queue.push(edge.clone());
            }
            if edge.to == node_id && !self.visited.contains(&edge.from) {
                self.priority_queue.push(edge.clone());
            }
        }
    }

    fn step(&mut self) {
        while let Some(edge) = self.priority_queue.pop() {
            let target = if self.visited.contains(&edge.from) {
                edge.to
            } else {
                edge.from
            };

            if !self.visited.contains(&target) {
                self.current_edge = Some(edge.clone());
                self.visited.insert(target);
                self.nodes[target].visited = true;
                self.add_edges_to_queue(target);

                if let Some(e) = self.edges.iter_mut().find(|e| *e == &edge) {
                    e.selected = true;
                }
                return;
            }
        }
        self.running = false;
    }
}

impl eframe::App for PrimVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            self.accumulated_time += ctx.input(|i| i.unstable_dt);
            if self.accumulated_time >= self.step_delay {
                self.accumulated_time = 0.0;
                self.step();
                ctx.request_repaint();
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::SidePanel::left("controls").show(ctx, |ui| {
                ui.heading("Prim ");
                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button(if self.running { "stop" } else { "start" }).clicked() {
                        self.running = !self.running;
                    }
                    if ui.button("reset").clicked() {
                        self.initialize_algorithm();
                    }
                });

                ui.add(egui::Slider::new(&mut self.step_delay, 0.1..=2.0).text("Step interval"));
            });

            let painter = ui.painter();

           
            for edge in &self.edges {
                let from_pos = self.nodes[edge.from].position;
                let to_pos = self.nodes[edge.to].position;
                
                let color = if edge.selected {
                    Color32::GREEN
                } else if Some(edge) == self.current_edge.as_ref() {
                    Color32::YELLOW
                } else {
                    Color32::from_gray(100)
                };

                painter.line_segment([from_pos, to_pos], (2.5, color));

                
                let mid_point = {
                    let from_vec = from_pos.to_vec2();
                    let to_vec = to_pos.to_vec2();
                    (from_vec + to_vec) / 2.0
                };

                painter.text(
                    mid_point.to_pos2(),
                    egui::Align2::CENTER_CENTER,
                    edge.weight.to_string(),
                    egui::FontId::monospace(14.0),
                    Color32::RED,
                );
            }

          
            for node in &self.nodes {
                let color = if node.visited {
                    Color32::from_rgb(100, 200, 255)
                } else {
                    Color32::from_gray(80)
                };

                painter.circle_filled(node.position, 20.0, color);
                painter.text(
                    node.position,
                    egui::Align2::CENTER_CENTER,
                    node.id.to_string(),
                    egui::FontId::monospace(18.0),
                    Color32::WHITE,
                );
            }

           
            egui::Window::new("Algorithm status").show(ctx, |ui| {
                ui.label(format!("visited node: {}/{}", self.visited.len(), self.nodes.len()));
                ui.label(format!("unvisited in queue: {}", self.priority_queue.len()));
                if let Some(edge) = &self.current_edge {
                    ui.label(format!("now: {} ↔ {} (weight: {})", edge.from, edge.to, edge.weight));
                }
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
   let _ = eframe::run_native(
        "Prim",
        options,
        Box::new(|_cc| Box::new(PrimVisualizer::default())),
    );
}