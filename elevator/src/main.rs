#[derive(Debug)]
/// コントローラが反応する必要があるエレベーター システム内のイベント。
enum Event {
    Lobby { floor: i32, dir: Direction },
    Arrived { floor: i32 },
    DoorOpened,
    DoorClosed,
    Car { floor: i32 },
}

/// 運転方向。
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// かごが所定の階に到着した。
fn car_arrived(floor: i32) -> Event {
    Event::Arrived { floor }
}

/// かごのドアが開いた。
fn car_door_opened() -> Event {
    Event::DoorOpened
}

/// かごのドアが閉まった。
fn car_door_closed() -> Event {
    Event::DoorClosed
}

/// 所定の階のエレベーター ロビーで方向ボタンが押された。
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::Lobby { floor, dir }
}

/// エレベーターのかごの階数ボタンが押された。
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::Car { floor }
}

fn main() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}
