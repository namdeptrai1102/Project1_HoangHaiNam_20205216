use iced::executor; // import thư viện
use iced::alignment;
use iced::theme::{self, Theme};
use iced::widget::canvas::{
    stroke, Cache, Cursor, Geometry, LineCap, Path, Stroke, Text
};
use iced::Renderer;
use iced::widget::{canvas, container, button, column, row, text};
use iced::{
    Application, Color, Command, Element, Length, Point, Rectangle, Settings,
    Subscription,  Vector, Alignment
};
use std::time::{Duration, Instant};
use iced::widget::pick_list;
use iced::widget::vertical_space;
pub fn main() -> iced::Result {
    Clock::run(Settings {
        antialiasing: true, //chống răng cưa
        ..Settings::default() 
    })
}
// Clock
struct Clock { 
    now: time::OffsetDateTime,
    clock: Cache,
    duration: Duration,
    state: State,
    selected_country: Option<Country>,
}
//Enum for stop watch
enum State {
    Idle,
    Ticking { last_tick: Instant },
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(time::OffsetDateTime),// https://docs.rs/time/latest/time/struct.OffsetDateTime.html
    TickStopWatch(Instant),
    Toggle,
    Reset,
    CountrySelected(Country),
}

impl Application for Clock { 
    type Executor = executor::Default;//thực thi mặc định
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) { //khởi tạo ứng dụng
        (
            Clock {
                now: time::OffsetDateTime::now_local()// lấy ngày giờ theo khu vực
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),//tạo 1 OffsetDateTime mới với ngày và giờ hiện tại theo UTC
                clock: Default::default(),
                duration: Duration::default(),
                state: State::Idle,
                selected_country: Option:: default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Clock (Hoàng Hải Nam - 20205216)")
        
    }

    fn update(&mut self, message: Message) -> Command<Message> { 
        match message {
            Message::Tick(local_time) => {
                let now = local_time;
                if now != self.now {
                    self.now = now;
                    self.clock.clear(); //xóa để refesh
                }
            }
            Message::Toggle => match self.state {
                State::Idle => {
                    self.state = State::Ticking {
                        last_tick: Instant::now(),
                    };
                }
                State::Ticking { .. } => {
                    self.state = State::Idle;
                }
            },
            Message::TickStopWatch(now) => {
                if let State::Ticking { last_tick } = &mut self.state {
                    self.duration += now - *last_tick;
                    *last_tick = now;
                }
                // Bổ sung trạng thái clock khi stopwatch đang chạy
                let now_clock = time::OffsetDateTime::now_local().unwrap_or_else(|_| time::OffsetDateTime::now_utc());
                if now_clock != self.now {
                    self.now = now_clock;
                    self.clock.clear();
                }
            }
            Message::Reset => {
                self.duration = Duration::default();
            }
            Message::CountrySelected(country) => {
               self.selected_country = Some(country);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        //Clock
        let canvas = canvas(self as &Self)
            .width(Length::Fill)
            .height(Length::Fill);

        // Stopwatch
        const MINUTE: u64 = 60;
        const HOUR: u64 = 60 * MINUTE;

        let seconds = self.duration.as_secs();
            // Chuỗi số
        let duration: iced::widget::text::Text<'_, Renderer> = text(format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            self.duration.subsec_millis() / 10,
        ))
        .size(60);
            // Thiết kế hình dáng nút
        let button = |label| {
            button(
                text(label).horizontal_alignment(alignment::Horizontal::Center),
            )
            .padding(15)
            .width(120)
        };
            // Nút start/stop
        let toggle_button = {
            let label = match self.state {
                State::Idle => "Start",
                State::Ticking { .. } => "Stop",
            };

            button(label).on_press(Message::Toggle)
        };
            // Nút reset
        let reset_button = button("Reset")
            .style(theme::Button::Destructive)
            .on_press(Message::Reset);
            // Sắp xếp vị trí stopwatch
        let controls = row![toggle_button, reset_button].spacing(30);
        let content1 = column![duration, controls]
            .align_items(Alignment::Center)
            .spacing(30);
        //Pick_list
        let pick_list = pick_list(
            &Country::ALL[..],
            self.selected_country,
            Message::CountrySelected,
        )
        .placeholder("Choose a country...");
        // Sắp xếp vị trí cột phải
        let content2 = column![
            content1,
            "Where do you want to see the time?",
            pick_list,
            vertical_space(350),
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(15);
        // Sắp xếp toàn màn hình
        let display = row![canvas, content2]
            .align_items(Alignment::Center)
            .spacing(20);
        container(display)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(30)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Idle => {
                iced::time::every(std::time::Duration::from_millis(500)).map(|_| {//đồng bộ mỗi 500 mili giây
                    Message::Tick(time::OffsetDateTime::now_local().unwrap_or_else(|_| time::OffsetDateTime::now_utc()),)})
            },
            State::Ticking { .. } => {
                iced::time::every(std::time::Duration::from_millis(10)).map(Message::TickStopWatch)
            }
        }
    }
}

impl<Message> canvas::Program<Message> for Clock {
    type State = ();
    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
            let clock = self.clock.draw(bounds.size(), |frame| { //
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;
            // Vẽ thân đồng hồ
            let background = Path::circle(center, radius);
            frame.fill(&background, Color::from_rgb8(0xC0, 0xC0, 0xC0));
            let three: f32 = 3.0;
            let cos60: f32 = 0.5;
            let sin60: f32 = three.sqrt()/2.0;
            let x0: f32 =  frame.width() / 2.0 - radius / 11.5;
            let y0: f32 = frame.height() / 2.0 - radius / 5.5;
            let num = Text{
                color: Color::BLACK,
                size: radius /3.0,
                ..Text::default()
            };
            // Vẽ số trên đồng hồ
            frame.fill_text(Text {content: format!("1"),position: Point::new( x0 + radius*0.8*cos60 ,y0 - radius*0.8*sin60),..num});
            frame.fill_text(Text {content: format!("2"),position: Point::new( x0 + radius*0.8*sin60 ,y0 - radius*0.8*cos60),..num });
            frame.fill_text(Text {content: format!("3"),position: Point::new(x0 + radius*0.8,y0),..num});
            frame.fill_text(Text {content: format!("4"),position: Point::new( x0 + radius*0.8*sin60 ,y0 + radius*0.8*cos60),..num });
            frame.fill_text(Text {content: format!("5"),position: Point::new( x0 + radius*0.8*cos60 ,y0 + radius*0.8*sin60),..num});
            frame.fill_text(Text {content: format!("6"),position: Point::new(x0, y0 + radius*0.8),..num});
            frame.fill_text(Text {content: format!("7"),position: Point::new( x0 - radius*0.8*cos60 ,y0 + radius*0.8*sin60),..num});
            frame.fill_text(Text {content: format!("8"),position: Point::new( x0 - radius*0.8*sin60 ,y0 + radius*0.8*cos60),..num });
            frame.fill_text(Text {content: format!("9"),position: Point::new(x0 - radius*0.8,y0),..num});
            frame.fill_text(Text {content: format!("10"),position: Point::new( x0 - radius*0.8*sin60 - radius*0.07,y0 - radius*0.8*cos60),..num });
            frame.fill_text(Text {content: format!("11"),position: Point::new( x0 - radius*0.8*cos60 - radius*0.07,y0 - radius*0.8*sin60),..num});
            frame.fill_text(Text {content: format!("12"),position: Point::new(x0 - radius*0.07, y0 - radius*0.8),..num});
            // Thiết kế kim đồng hồ
            let short_hand =
                Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius)); //ORIGIN là điểm chính giữa (0,0)
            let long_hand =
                Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));
            let width = radius / 100.0; //độ rộng kim đồng hồ
            let thin_stroke = || -> Stroke {
                Stroke {
                    width,
                    style: stroke::Style::Solid(Color::BLACK),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };
            let wide_stroke = || -> Stroke {
                Stroke {
                    width: width * 3.0,
                    style: stroke::Style::Solid(Color::BLACK),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };
            // Vẽ kim đồng hồ
            frame.translate(Vector::new(center.x, center.y));
                // Kim giây
            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.second(), 60));
                frame.stroke(&long_hand, thin_stroke());
            });
                // Kim giờ
            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.hour(), 12));
                frame.stroke(&short_hand, wide_stroke());
            });
                // Kim phút
            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.minute(), 60));
                frame.stroke(&long_hand, wide_stroke());
            });
            // Thiết kế tên nước
            let country = Text{
                color: Color::from_rgb8(52,138,148),
                size: radius /5.0,
                ..Text::default()
            };
            frame.fill_text(Text {content: format!("VIET NAM"),position: Point::new( - radius*0.36 ,- radius*0.2),color: Color::BLACK,..country});
            frame.fill_text(Text {content: format!("{:02} {}, {}",self.now.month(),self.now.day(),self.now.year()),position: Point::new( - radius*0.55 ,radius*0.0),color: Color::BLACK,..country});
            // Hiển thị tên nước và thời gian
            match self.selected_country{
                Some(Country::US) =>  frame.fill_text(Text {content: format!("US \n{:02}:{:02}:{:02}",time_zone(self.now.hour(),-5),self.now.minute(),self.now.second()),position: Point::new(1.7*radius, 0.4*radius),..country}),
                Some(Country::ENGLAND) =>  frame.fill_text(Text {content: format!("England \n{:02}:{:02}:{:02}",time_zone(self.now.hour(),0),self.now.minute(),self.now.second()),position: Point::new(1.7*radius, 0.4*radius),..country}),
                Some(Country::FRANCE) =>  frame.fill_text(Text {content: format!("France \n{:02}:{:02}:{:02}",time_zone(self.now.hour(),1),self.now.minute(),self.now.second()),position: Point::new(1.7*radius, 0.4*radius),..country}),
                Some(Country::CANADA) =>  frame.fill_text(Text {content: format!("Canada \n{:02}:{:02}:{:02}",time_zone(self.now.hour(),-6),self.now.minute(),self.now.second()),position: Point::new(1.7*radius, 0.4*radius),..country}),
                Some(Country::AUSTRALIA) =>  frame.fill_text(Text {content: format!("Australia \n{:02}:{:02}:{:02}",time_zone(self.now.hour(),11),self.now.minute(),self.now.second()),position: Point::new(1.7*radius, 0.4*radius),..country}),
                Some(Country::FINLAND) =>  frame.fill_text(Text {content: format!("Finland \n{:02}:{:02}:{:02}",time_zone(self.now.hour(),2),self.now.minute(),self.now.second()),position: Point::new(1.7*radius, 0.4*radius),..country}),
                Some(Country::RUSSIA) =>  frame.fill_text(Text {content: format!("Russia \n{:02}:{:02}:{:02}",time_zone(self.now.hour(),3),self.now.minute(),self.now.second()),position: Point::new(1.7*radius, 0.4*radius),..country}),

                None => frame.fill_text(Text {content: format!("Viet Nam \n{:02}:{:02}:{:02}",time_zone(self.now.hour(),7),self.now.minute(),self.now.second()),position: Point::new(1.7*radius, 0.4*radius),..country}),
            }
        });
        vec![clock]
    }
}
// Hàm chuyển đổi múi giờ
fn time_zone(vn: u8, tz: i8) -> i8{                         
    let  sum = vn as i8 -7 + tz;
    let foreign_time;
    if sum >= 24{
        foreign_time = sum -24;
    }else if sum < 0{
        foreign_time = sum + 24;
    } else {
        foreign_time = sum
    }
    foreign_time
}
// Hàm xoay kim đồng hồ
fn hand_rotation(n: u8, total: u8) -> f32 {
    let turns = n as f32 / total as f32;
    2.0 * std::f32::consts::PI * turns
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Country {
    US,
    ENGLAND,
    FRANCE,
    CANADA,
    AUSTRALIA,
    FINLAND,
    RUSSIA,
}
// Danh sách tên nước cho pick list
impl Country {
    const ALL: [Country; 7] = [
        Country::US,
        Country::ENGLAND,
        Country::FRANCE,
        Country::CANADA,
        Country::AUSTRALIA,
        Country::FINLAND,
        Country::RUSSIA,
    ];
}
//Tùy chọn tên nước và múi giờ trên pick list
impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Country::US => "US(CT): UTC-5",
                Country::ENGLAND => "England(GMT): UTC+0",
                Country::FRANCE => "France(CET): UTC+1",
                Country::CANADA => "Canada(CST): UTC-6",
                Country::AUSTRALIA => "Australia(AEDT): UTC+11",
                Country::FINLAND => "Finland(EET): UTC+2",
                Country::RUSSIA => "Russia(MSK): UTC+3",
            }
        )
    }
}