# lab02

## 软2304-景奕瑞-20232241467

## egui示例
### 结构体定义

```
pub struct BasicApp{
g:egui_graphs::Graph,
}
```

### 结构体函数的定义

```
impl BasicApp{
fn new(_:&eframe::CreationContext<'_>')-
	let g=generate_graph();
	self{g:egui_graphs::Graph::from(&g)}
}
```

```
let g = generate_graph();
```

生成一个包含3个节点和3条边的3角形图

```
self{g:egui_graphs::Graph::from(&g)
```

将自定义图转换为egui兼容模式

### 图生成函数

```
fn generate_graph() -> StableGraph<(), ()> {
    let mut g = StableGraph::new();
    let a = g.add_node(()); 
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, b, ());
    g.add_edge(b, c, ());
    g.add_edge(c, a, ());
    g
}
```

使用new函数不断生成新的边和节点

### 图的绘制

```
fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(&mut DefaultGraphView::new(&mut self.g));
        });
    }
```

defaultGraphView:egui_graphs提供的默认图视图组件自动处理边的节点绘制、布局和交互

centralpanel：egui的中心面板组件



### 主函数

```
fn main() {
    run_native(
        "egui_graphs_basic_demo", 
        NativeOptions::default(),  
        Box::new(|cc| Ok(Box::new(BasicApp::new(cc)))), 
    )
    .unwrap();
}
```

设置默认的窗口配置

### 生成窗口

```
fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        Window::new("graph").show(ctx, |ui| {
            ui.add(&mut DefaultGraphView::new(&mut self.g));
        });
    }
}
```

使用window来生成一个窗口，其中可以自定义命名

## 使用egui实现prim算法

### 定义节点

```
struct Node {
    id: usize,
    position: Pos2,
    visited: bool,
}
```

定义节点的结构体保存其id，位置，以及是否被访问

### 定义边

```
struct Edge {
    from: usize,
    to: usize,
    weight: i32,
    selected: bool,
}
```

定义边的结构体，其中有边连接接待你的节点id，变得权重以及边是否被挑选中

### 边的比较

```
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}
```

比较边的权重，通过反转比较顺序与binaryheap结合来定义一个最小堆

### 定义prim算法的可视化

```
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
```

定义节点，边，优先队列，已经访问的节点集合，当前正在处理的边的集合来保存算法运行到哪一步，同时定义算法运行状态的记录，以及运行时间来实现自动进行prim算法的可视化。

### 初始化节点

```
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

```

初始化5个节点，其中每个节点都在以（300，300）为圆心200为半径的圆上，并且将原5等分来计算不同点的坐标位置。并且将每个节点设置为unvistied

### 初始化边

```
 let edges = vec![
            Edge { from: 0, to: 1, weight: 2, selected: false },
            Edge { from: 0, to: 3, weight: 6, selected: false },
            Edge { from: 1, to: 2, weight: 3, selected: false },
            Edge { from: 1, to: 3, weight: 8, selected: false },
            Edge { from: 1, to: 4, weight: 5, selected: false },
            Edge { from: 2, to: 4, weight: 7, selected: false },
            Edge { from: 3, to: 4, weight: 9, selected: false },
        ];

```

定义7条边，每一条都保存有边连接的两个节点的id。将每条边设置为未被选中状态

### 将边加入队列

```
add_edges_to_queue(&mut self, node_id: usize) {
        for edge in &self.edges {
            if edge.from == node_id && !self.visited.contains(&edge.to) {
                self.priority_queue.push(edge.clone());
            }
            if edge.to == node_id && !self.visited.contains(&edge.from) {
                self.priority_queue.push(edge.clone());
            }
        }
```

判断边的起始点是为点的id，并且点处于未被访问的状态，此时将边加优先限队列当中。让后通过双向的检查来确保能正确的处理无向图。

### prim核心算法的实现

```
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
```

首先通过 

```
while let Some(edge) = self.priority_queue.pop()
```

来循环取出优先队列中权最小的边。

然后通过

```
let target = if self.visited.contains(&edge.from) {
            edge.to
        } else {
            edge.from
        };
```

来找到边中的节点

接着通过

```
if !self.visited.contains(&target) {
         
            self.current_edge = Some(edge.clone());

            
            self.visited.insert(target);
            self.nodes[target].visited = true; 

     
            self.add_edges_to_queue(target);

         
            if let Some(e) = self.edges.iter_mut().find(|e| *e == &edge) {
                e.selected = true;
            }
```

来判断目标的节点是否被访问，然后更新访问集合，更新节点的状态，方便后续被染色。

同时通过

```
 self.add_edges_to_queue(target);
```

来将新的节点加入队列。

同时标记被取出的边已经被选中，标记其已被选中状态，方便后续的可视化染色

### 可视化处理

####  自动化实现算法

```
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            self.accumulated_time += ctx.input(|i| i.unstable_dt);
            if self.accumulated_time >= self.step_delay {
                self.accumulated_time = 0.0;
                self.step();
                ctx.request_repaint();
            }
        }
```

首先判断算法是否在运行

如果算法在巡行中则不断的累加时间。当时间超过所设定的间隔就执行重置计时器，并且重新绘制ui。

#### ui布局以及实现ui控制

```
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

```

首先使用centralpanel设置主内容区域，然后设置sidepanel设置左边的边栏为控制栏。通过嵌套关系，将控制面版嵌套在主内容区域中。

然后设置button，通过if else判断程序是否在运行，如果正在运行则设置为stop，不在运行则设置为start。然后设置按键reset。按下后重新开始算法。

通过slider设置滑动条，将其范围设置到0.1-2.0之间来控制整个算法自动化运行的时候的时间间隔

#### 边绘制

```
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

```

首先循环遍历所有边，取出边所连接的两个节点。然后确定边的颜色。如果是selected，则说明其已经被选中，将其绘制为绿色。如果为some中的则说明其正在被处理，绘制为蓝色。剩下的则为普通边，绘制为灰色。然后使用line_segment来绘制指定了颜色和线宽的边。

```
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
                    Color32::WHITE,
                );
            }
```

同时获取边的中间位置，在边的中心位置写text文本表示不同边的权重。

#### 绘制节点

```
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

```

循环取出点。如果点时被访问过的则将其设置为蓝色。否则设置为灰色。同时绘制圆形来表示点。在点上通过text文本写上是哪个点

#### 算法进程

```
 egui::Window::new("Algorithm status").show(ctx, |ui| {
                ui.label(format!("visited node: {}/{}", self.visited.len(), self.nodes.len()));
                ui.label(format!("unvisited in queue: {}", self.priority_queue.len()));
                if let Some(edge) = &self.current_edge {
                    ui.label(format!("now: {} ↔ {} (weight: {})", edge.from, edge.to, edge.weight));
                }
            });
```

通过label来说明已经访问的节点数量以及全部的节点的数量。然后说明优先队列中最小的边的权重。

同时通过some判断中是否有数据，如果有则出去取出的边的权重以及其所联通的两个节点。