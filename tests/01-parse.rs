use machine_macro::machine;

machine!{
  "id": "Light Switch",
  "initial": "Off",
  "states": {
    "Off": {
      "entry": ["one", "two"],
      "on": {
        "Switch On": [
            {
              "target": "On",
              "cond": "someCond"
            },
            {
                "target": "On"
            }
        ]
      }
    },
    "On": {
      "on": {
        "Switch Off": {
          "target": "Off"
        }
      }
    }
  }
}

fn main() {}
