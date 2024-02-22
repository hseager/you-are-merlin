const states = [
  "travelling",
  "visiting"
];

const actions = [
  "Travel",
  "Explore",
  "Attack",
  "Give Item"
]

[
  {
    type: "location",
    name: "Camelot Castle",
    description: "Majestic fortress atop rugged cliffs, echoing tales of chivalry.",
    quest: {
      character: "Morgana",
      item: "Nimueh's Dark Tome"
    }
  },
  {
    type: "location",
    name: "Darkling Woods",
    description: "Eerie woods of looming darkness, where shadows dance and secrets whisper.",
    encounters: [
      {
        type: "battle",
        enemy: {
          name: "Uther's Undead Knight",
          life: 20,
          attack: 5,
        },
      }
    ],
  }
]