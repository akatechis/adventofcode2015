use std::collections::HashSet;
use std::collections::HashMap;

type Coord = (usize, usize);

#[derive(Debug, PartialEq)]
pub enum Action {
  On, Off, Toggle
}

type SoftLight = usize;

pub struct BinLightGrid {
  lights: HashSet<Coord>
}

pub struct SoftLightGrid {
  lights: HashMap<Coord, SoftLight>
}

impl BinLightGrid {
  fn new () -> Self {
    BinLightGrid {
      lights: HashSet::new()
    }
  }
}

impl SoftLightGrid {
  fn new () -> Self {
    SoftLightGrid {
      lights: HashMap::new()
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Instr {
  action: Action,
  from: Coord,
  to: Coord
}

pub trait LightGrid {
  fn perform_action(&mut self, row: usize, col: usize, action: &Action);
  fn light_magnitude(&self) -> usize;
}

impl LightGrid for BinLightGrid {
  fn perform_action (&mut self, r: usize, c: usize, action: &Action) {
    match action {
      Action::On => {
        self.lights.insert((r, c));
      },
      Action::Off => {
        self.lights.remove(&(r, c));
      },
      Action::Toggle => {
        if self.lights.contains(&(r,c)) {
          self.lights.remove(&(r,c));
        } else {
          self.lights.insert((r,c));
        }
      }
    }
  }

  fn light_magnitude (&self) -> usize {
    self.lights.len()
  }
}

impl SoftLightGrid {
  fn dim_light (&mut self, r: usize, c: usize) {
    let mut remove = false;
    let mut decrement = false;

    { // decision phase
      let light = self.lights.get(&(r, c));
      if let Some(lvl) = light {
        if *lvl == 1 {
          remove = true;
        } else {
          decrement = true;
        }
      }
    }

    // handle phase
    if remove {
      self.lights.remove_entry(&(r, c));
    } else if decrement {
      let lvl = self.lights.get_mut(&(r, c)).unwrap();
      *lvl -= 1;
    }
  }
}

impl LightGrid for SoftLightGrid {
  fn perform_action (
    &mut self,
    r: usize,
    c: usize,
    action: &Action
  ) {
    match action {
      Action::On => {
        // add 1 to the brightness
        let light = self.lights.entry((r,c))
        .or_insert(0);
        *light += 1;
      },
      Action::Off => {
        self.dim_light(r, c);
      },
      Action::Toggle => {
        // add 2 to brightness
        let light = self.lights.entry((r,c))
        .or_insert(0);
        *light += 2;
      }
    };
  }

  fn light_magnitude (&self) -> usize {
    let mut lights = 0;
    for (_, light) in self.lights.iter() {
      lights += *light;
    }
    lights
  }
}

fn perform_instruction <G: LightGrid> (
  lights: &mut G,
  &Instr{ ref action, from, to }: &Instr,
) {
  // println!("Rect: ({:?}, {:?})", from, to);
  let (f_x, f_y) = from;
  let (t_x, t_y) = to;
  for r in f_x..=t_x {
    for c in f_y..=t_y {
      // println!("Cell: ({}, {})", r, c);
      lights.perform_action(r, c, &action);
      // println!("Magnitude: {}", lights.light_magnitude());
    }
  }
}

fn perform_instructions <G: LightGrid> (
  lights: &mut G,
  steps: &Vec<Instr>
) {
  for step in steps {
    perform_instruction(lights, &step);
  }
}

fn part_1 (instructions: &Vec<Instr>) {
  let mut grid = BinLightGrid::new();
  perform_instructions(&mut grid, &instructions);

  let lights = grid.light_magnitude();
  println!("Light magnitude for a binary light grid: {:?}", lights);
}

fn part_2 (instructions: &Vec<Instr>) {
  let mut grid = SoftLightGrid::new();
  perform_instructions(&mut grid, &instructions);

  let lights = grid.light_magnitude();
  println!("Light magnitude for a soft light grid: {:?}", lights);
}

pub fn main () {
  let instructions = data::create_instructions();
  part_1(&instructions);
  part_2(&instructions);
}

mod data {
  use super::Action;
  use super::*;
  use std::iter::Iterator;

  pub fn parse(raw: &String) -> Instr {
    let mut tokens = raw.split_whitespace();

    let verb = tokens.next().unwrap();
    let action = {
      if verb == "turn" {
        let kind = tokens.next().unwrap();

        if kind == "on" {
          Action::On
        } else if kind == "off" {
          Action::Off
        } else {
          panic!("Not sure how to turn: {}", raw);
        }
      } else if verb == "toggle" {
        Action::Toggle
      } else {
        panic!("Not sure what action kind this is: {}", raw);
      }
    };

    let from = {
      let mut nums = tokens.next().unwrap().split(',');
      let x = nums.next().unwrap().parse::<usize>().unwrap();
      let y = nums.next().unwrap().parse::<usize>().unwrap();
      (x, y)
    };

    tokens.next();

    let to = {
      let mut nums = tokens.next().unwrap().split(',');
      let x = nums.next().unwrap().parse::<usize>().unwrap();
      let y = nums.next().unwrap().parse::<usize>().unwrap();
      (x, y)
    };

    Instr {
      action, from, to
    }
  }

  pub fn create_instructions() -> Vec<Instr> {
    raw().iter().map(parse).collect()
  }

  fn raw() -> Vec<String> {
    vec![
      "turn on 887,9 through 959,629".to_string(),
      "turn on 454,398 through 844,448".to_string(),
      "turn off 539,243 through 559,965".to_string(),
      "turn off 370,819 through 676,868".to_string(),
      "turn off 145,40 through 370,997".to_string(),
      "turn off 301,3 through 808,453".to_string(),
      "turn on 351,678 through 951,908".to_string(),
      "toggle 720,196 through 897,994".to_string(),
      "toggle 831,394 through 904,860".to_string(),
      "toggle 753,664 through 970,926".to_string(),
      "turn off 150,300 through 213,740".to_string(),
      "turn on 141,242 through 932,871".to_string(),
      "toggle 294,259 through 474,326".to_string(),
      "toggle 678,333 through 752,957".to_string(),
      "toggle 393,804 through 510,976".to_string(),
      "turn off 6,964 through 411,976".to_string(),
      "turn off 33,572 through 978,590".to_string(),
      "turn on 579,693 through 650,978".to_string(),
      "turn on 150,20 through 652,719".to_string(),
      "turn off 782,143 through 808,802".to_string(),
      "turn off 240,377 through 761,468".to_string(),
      "turn off 899,828 through 958,967".to_string(),
      "turn on 613,565 through 952,659".to_string(),
      "turn on 295,36 through 964,978".to_string(),
      "toggle 846,296 through 969,528".to_string(),
      "turn off 211,254 through 529,491".to_string(),
      "turn off 231,594 through 406,794".to_string(),
      "turn off 169,791 through 758,942".to_string(),
      "turn on 955,440 through 980,477".to_string(),
      "toggle 944,498 through 995,928".to_string(),
      "turn on 519,391 through 605,718".to_string(),
      "toggle 521,303 through 617,366".to_string(),
      "turn off 524,349 through 694,791".to_string(),
      "toggle 391,87 through 499,792".to_string(),
      "toggle 562,527 through 668,935".to_string(),
      "turn off 68,358 through 857,453".to_string(),
      "toggle 815,811 through 889,828".to_string(),
      "turn off 666,61 through 768,87".to_string(),
      "turn on 27,501 through 921,952".to_string(),
      "turn on 953,102 through 983,471".to_string(),
      "turn on 277,552 through 451,723".to_string(),
      "turn off 64,253 through 655,960".to_string(),
      "turn on 47,485 through 734,977".to_string(),
      "turn off 59,119 through 699,734".to_string(),
      "toggle 407,898 through 493,955".to_string(),
      "toggle 912,966 through 949,991".to_string(),
      "turn on 479,990 through 895,990".to_string(),
      "toggle 390,589 through 869,766".to_string(),
      "toggle 593,903 through 926,943".to_string(),
      "toggle 358,439 through 870,528".to_string(),
      "turn off 649,410 through 652,875".to_string(),
      "turn on 629,834 through 712,895".to_string(),
      "toggle 254,555 through 770,901".to_string(),
      "toggle 641,832 through 947,850".to_string(),
      "turn on 268,448 through 743,777".to_string(),
      "turn off 512,123 through 625,874".to_string(),
      "turn off 498,262 through 930,811".to_string(),
      "turn off 835,158 through 886,242".to_string(),
      "toggle 546,310 through 607,773".to_string(),
      "turn on 501,505 through 896,909".to_string(),
      "turn off 666,796 through 817,924".to_string(),
      "toggle 987,789 through 993,809".to_string(),
      "toggle 745,8 through 860,693".to_string(),
      "toggle 181,983 through 731,988".to_string(),
      "turn on 826,174 through 924,883".to_string(),
      "turn on 239,228 through 843,993".to_string(),
      "turn on 205,613 through 891,667".to_string(),
      "toggle 867,873 through 984,896".to_string(),
      "turn on 628,251 through 677,681".to_string(),
      "toggle 276,956 through 631,964".to_string(),
      "turn on 78,358 through 974,713".to_string(),
      "turn on 521,360 through 773,597".to_string(),
      "turn off 963,52 through 979,502".to_string(),
      "turn on 117,151 through 934,622".to_string(),
      "toggle 237,91 through 528,164".to_string(),
      "turn on 944,269 through 975,453".to_string(),
      "toggle 979,460 through 988,964".to_string(),
      "turn off 440,254 through 681,507".to_string(),
      "toggle 347,100 through 896,785".to_string(),
      "turn off 329,592 through 369,985".to_string(),
      "turn on 931,960 through 979,985".to_string(),
      "toggle 703,3 through 776,36".to_string(),
      "toggle 798,120 through 908,550".to_string(),
      "turn off 186,605 through 914,709".to_string(),
      "turn off 921,725 through 979,956".to_string(),
      "toggle 167,34 through 735,249".to_string(),
      "turn on 726,781 through 987,936".to_string(),
      "toggle 720,336 through 847,756".to_string(),
      "turn on 171,630 through 656,769".to_string(),
      "turn off 417,276 through 751,500".to_string(),
      "toggle 559,485 through 584,534".to_string(),
      "turn on 568,629 through 690,873".to_string(),
      "toggle 248,712 through 277,988".to_string(),
      "toggle 345,594 through 812,723".to_string(),
      "turn off 800,108 through 834,618".to_string(),
      "turn off 967,439 through 986,869".to_string(),
      "turn on 842,209 through 955,529".to_string(),
      "turn on 132,653 through 357,696".to_string(),
      "turn on 817,38 through 973,662".to_string(),
      "turn off 569,816 through 721,861".to_string(),
      "turn on 568,429 through 945,724".to_string(),
      "turn on 77,458 through 844,685".to_string(),
      "turn off 138,78 through 498,851".to_string(),
      "turn on 136,21 through 252,986".to_string(),
      "turn off 2,460 through 863,472".to_string(),
      "turn on 172,81 through 839,332".to_string(),
      "turn on 123,216 through 703,384".to_string(),
      "turn off 879,644 through 944,887".to_string(),
      "toggle 227,491 through 504,793".to_string(),
      "toggle 580,418 through 741,479".to_string(),
      "toggle 65,276 through 414,299".to_string(),
      "toggle 482,486 through 838,931".to_string(),
      "turn off 557,768 through 950,927".to_string(),
      "turn off 615,617 through 955,864".to_string(),
      "turn on 859,886 through 923,919".to_string(),
      "turn on 391,330 through 499,971".to_string(),
      "toggle 521,835 through 613,847".to_string(),
      "turn on 822,787 through 989,847".to_string(),
      "turn on 192,142 through 357,846".to_string(),
      "turn off 564,945 through 985,945".to_string(),
      "turn off 479,361 through 703,799".to_string(),
      "toggle 56,481 through 489,978".to_string(),
      "turn off 632,991 through 774,998".to_string(),
      "toggle 723,526 through 945,792".to_string(),
      "turn on 344,149 through 441,640".to_string(),
      "toggle 568,927 through 624,952".to_string(),
      "turn on 621,784 through 970,788".to_string(),
      "toggle 665,783 through 795,981".to_string(),
      "toggle 386,610 through 817,730".to_string(),
      "toggle 440,399 through 734,417".to_string(),
      "toggle 939,201 through 978,803".to_string(),
      "turn off 395,883 through 554,929".to_string(),
      "turn on 340,309 through 637,561".to_string(),
      "turn off 875,147 through 946,481".to_string(),
      "turn off 945,837 through 957,922".to_string(),
      "turn off 429,982 through 691,991".to_string(),
      "toggle 227,137 through 439,822".to_string(),
      "toggle 4,848 through 7,932".to_string(),
      "turn off 545,146 through 756,943".to_string(),
      "turn on 763,863 through 937,994".to_string(),
      "turn on 232,94 through 404,502".to_string(),
      "turn off 742,254 through 930,512".to_string(),
      "turn on 91,931 through 101,942".to_string(),
      "toggle 585,106 through 651,425".to_string(),
      "turn on 506,700 through 567,960".to_string(),
      "turn off 548,44 through 718,352".to_string(),
      "turn off 194,827 through 673,859".to_string(),
      "turn off 6,645 through 509,764".to_string(),
      "turn off 13,230 through 821,361".to_string(),
      "turn on 734,629 through 919,631".to_string(),
      "toggle 788,552 through 957,972".to_string(),
      "toggle 244,747 through 849,773".to_string(),
      "turn off 162,553 through 276,887".to_string(),
      "turn off 569,577 through 587,604".to_string(),
      "turn off 799,482 through 854,956".to_string(),
      "turn on 744,535 through 909,802".to_string(),
      "toggle 330,641 through 396,986".to_string(),
      "turn off 927,458 through 966,564".to_string(),
      "toggle 984,486 through 986,913".to_string(),
      "toggle 519,682 through 632,708".to_string(),
      "turn on 984,977 through 989,986".to_string(),
      "toggle 766,423 through 934,495".to_string(),
      "turn on 17,509 through 947,718".to_string(),
      "turn on 413,783 through 631,903".to_string(),
      "turn on 482,370 through 493,688".to_string(),
      "turn on 433,859 through 628,938".to_string(),
      "turn off 769,549 through 945,810".to_string(),
      "turn on 178,853 through 539,941".to_string(),
      "turn off 203,251 through 692,433".to_string(),
      "turn off 525,638 through 955,794".to_string(),
      "turn on 169,70 through 764,939".to_string(),
      "toggle 59,352 through 896,404".to_string(),
      "toggle 143,245 through 707,320".to_string(),
      "turn off 103,35 through 160,949".to_string(),
      "toggle 496,24 through 669,507".to_string(),
      "turn off 581,847 through 847,903".to_string(),
      "turn on 689,153 through 733,562".to_string(),
      "turn on 821,487 through 839,699".to_string(),
      "turn on 837,627 through 978,723".to_string(),
      "toggle 96,748 through 973,753".to_string(),
      "toggle 99,818 through 609,995".to_string(),
      "turn on 731,193 through 756,509".to_string(),
      "turn off 622,55 through 813,365".to_string(),
      "turn on 456,490 through 576,548".to_string(),
      "turn on 48,421 through 163,674".to_string(),
      "turn off 853,861 through 924,964".to_string(),
      "turn off 59,963 through 556,987".to_string(),
      "turn on 458,710 through 688,847".to_string(),
      "toggle 12,484 through 878,562".to_string(),
      "turn off 241,964 through 799,983".to_string(),
      "turn off 434,299 through 845,772".to_string(),
      "toggle 896,725 through 956,847".to_string(),
      "turn on 740,289 through 784,345".to_string(),
      "turn off 395,840 through 822,845".to_string(),
      "turn on 955,224 through 996,953".to_string(),
      "turn off 710,186 through 957,722".to_string(),
      "turn off 485,949 through 869,985".to_string(),
      "turn on 848,209 through 975,376".to_string(),
      "toggle 221,241 through 906,384".to_string(),
      "turn on 588,49 through 927,496".to_string(),
      "turn on 273,332 through 735,725".to_string(),
      "turn on 505,962 through 895,962".to_string(),
      "toggle 820,112 through 923,143".to_string(),
      "turn on 919,792 through 978,982".to_string(),
      "toggle 489,461 through 910,737".to_string(),
      "turn off 202,642 through 638,940".to_string(),
      "turn off 708,953 through 970,960".to_string(),
      "toggle 437,291 through 546,381".to_string(),
      "turn on 409,358 through 837,479".to_string(),
      "turn off 756,279 through 870,943".to_string(),
      "turn off 154,657 through 375,703".to_string(),
      "turn off 524,622 through 995,779".to_string(),
      "toggle 514,221 through 651,850".to_string(),
      "toggle 808,464 through 886,646".to_string(),
      "toggle 483,537 through 739,840".to_string(),
      "toggle 654,769 through 831,825".to_string(),
      "turn off 326,37 through 631,69".to_string(),
      "turn off 590,570 through 926,656".to_string(),
      "turn off 881,913 through 911,998".to_string(),
      "turn on 996,102 through 998,616".to_string(),
      "turn off 677,503 through 828,563".to_string(),
      "turn on 860,251 through 877,441".to_string(),
      "turn off 964,100 through 982,377".to_string(),
      "toggle 888,403 through 961,597".to_string(),
      "turn off 632,240 through 938,968".to_string(),
      "toggle 731,176 through 932,413".to_string(),
      "turn on 5,498 through 203,835".to_string(),
      "turn on 819,352 through 929,855".to_string(),
      "toggle 393,813 through 832,816".to_string(),
      "toggle 725,689 through 967,888".to_string(),
      "turn on 968,950 through 969,983".to_string(),
      "turn off 152,628 through 582,896".to_string(),
      "turn off 165,844 through 459,935".to_string(),
      "turn off 882,741 through 974,786".to_string(),
      "turn off 283,179 through 731,899".to_string(),
      "toggle 197,366 through 682,445".to_string(),
      "turn on 106,309 through 120,813".to_string(),
      "toggle 950,387 through 967,782".to_string(),
      "turn off 274,603 through 383,759".to_string(),
      "turn off 155,665 through 284,787".to_string(),
      "toggle 551,871 through 860,962".to_string(),
      "turn off 30,826 through 598,892".to_string(),
      "toggle 76,552 through 977,888".to_string(),
      "turn on 938,180 through 994,997".to_string(),
      "toggle 62,381 through 993,656".to_string(),
      "toggle 625,861 through 921,941".to_string(),
      "turn on 685,311 through 872,521".to_string(),
      "turn on 124,934 through 530,962".to_string(),
      "turn on 606,379 through 961,867".to_string(),
      "turn off 792,735 through 946,783".to_string(),
      "turn on 417,480 through 860,598".to_string(),
      "toggle 178,91 through 481,887".to_string(),
      "turn off 23,935 through 833,962".to_string(),
      "toggle 317,14 through 793,425".to_string(),
      "turn on 986,89 through 999,613".to_string(),
      "turn off 359,201 through 560,554".to_string(),
      "turn off 729,494 through 942,626".to_string(),
      "turn on 204,143 through 876,610".to_string(),
      "toggle 474,97 through 636,542".to_string(),
      "turn off 902,924 through 976,973".to_string(),
      "turn off 389,442 through 824,638".to_string(),
      "turn off 622,863 through 798,863".to_string(),
      "turn on 840,622 through 978,920".to_string(),
      "toggle 567,374 through 925,439".to_string(),
      "turn off 643,319 through 935,662".to_string(),
      "toggle 185,42 through 294,810".to_string(),
      "turn on 47,124 through 598,880".to_string(),
      "toggle 828,303 through 979,770".to_string(),
      "turn off 174,272 through 280,311".to_string(),
      "turn off 540,50 through 880,212".to_string(),
      "turn on 141,994 through 221,998".to_string(),
      "turn on 476,695 through 483,901".to_string(),
      "turn on 960,216 through 972,502".to_string(),
      "toggle 752,335 through 957,733".to_string(),
      "turn off 419,713 through 537,998".to_string(),
      "toggle 772,846 through 994,888".to_string(),
      "turn on 881,159 through 902,312".to_string(),
      "turn off 537,651 through 641,816".to_string(),
      "toggle 561,947 through 638,965".to_string(),
      "turn on 368,458 through 437,612".to_string(),
      "turn on 290,149 through 705,919".to_string(),
      "turn on 711,918 through 974,945".to_string(),
      "toggle 916,242 through 926,786".to_string(),
      "toggle 522,272 through 773,314".to_string(),
      "turn on 432,897 through 440,954".to_string(),
      "turn off 132,169 through 775,380".to_string(),
      "toggle 52,205 through 693,747".to_string(),
      "toggle 926,309 through 976,669".to_string(),
      "turn off 838,342 through 938,444".to_string(),
      "turn on 144,431 through 260,951".to_string(),
      "toggle 780,318 through 975,495".to_string(),
      "turn off 185,412 through 796,541".to_string(),
      "turn on 879,548 through 892,860".to_string(),
      "turn on 294,132 through 460,338".to_string(),
      "turn on 823,500 through 899,529".to_string(),
      "turn off 225,603 through 483,920".to_string(),
      "toggle 717,493 through 930,875".to_string(),
      "toggle 534,948 through 599,968".to_string(),
      "turn on 522,730 through 968,950".to_string(),
      "turn off 102,229 through 674,529".to_string()
    ]
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use super::data::*;

  #[test]
  fn test_parsing_turn_on_works_correctly () {
    let i = parse(&String::from("turn on 887,9 through 959,629"));
    let expected = Instr {
      action: Action::On,
      from: (887,9),
      to: (959, 629)
    };

    assert_eq!(expected, i);
  }

  #[test]
  fn test_parsing_turn_off_works_correctly () {
    let i = parse(&String::from("turn off 632,991 through 774,998"));
    let expected = Instr {
      action: Action::Off,
      from: (632,991),
      to: (774, 998)
    };

    assert_eq!(expected, i);
  }

  #[test]
  fn test_parsing_toggle_works_correctly () {
    let i = parse(&String::from("toggle 717,493 through 930,875"));
    let expected = Instr {
      action: Action::Toggle,
      from: (717,493),
      to: (930, 875)
    };

    assert_eq!(expected, i);
  }

  #[test]
  fn test_binary_light_grid () {
    let mut grid = BinLightGrid::new();

    { // turn on some lights
      let action = Instr {
        action: Action::On,
        from: (0, 0),
        to: (4, 4)
      };
      perform_instruction(&mut grid, &action);
      assert_eq!(25, grid.light_magnitude());
    }

    { // they stay on
      let action = Instr {
        action: Action::On,
        from: (0, 0),
        to: (4, 4)
      };
      perform_instruction(&mut grid, &action);
      assert_eq!(25, grid.light_magnitude());
    }

    { // turn some more on with partial overlapping
      let action = Instr {
        action: Action::On,
        from: (2, 3),
        to: (5, 7)
      };
      perform_instruction(&mut grid, &action);
      assert_eq!(39, grid.light_magnitude());
    }

    { // toggle some off
      let action = Instr {
        action: Action::Toggle,
        from: (2, 2),
        to: (3, 3)
      };
      perform_instruction(&mut grid, &action);
      assert_eq!(35, grid.light_magnitude());
    }

    { // toggle some on/off
      let action = Instr {
        action: Action::Toggle,
        from: (5, 3),
        to: (7, 4)
      };
      perform_instruction(&mut grid, &action);
      assert_eq!(37, grid.light_magnitude());
    }

    { // toggle the first one off
      let action = Instr {
        action: Action::Off,
        from: (0, 0),
        to: (0, 0)
      };
      perform_instruction(&mut grid, &action);
      assert_eq!(36, grid.light_magnitude());
    }
  }

  #[test]
  fn test_soft_light_grid () {
    // let mut grid = SoftLightGrid::new();
  }
}
