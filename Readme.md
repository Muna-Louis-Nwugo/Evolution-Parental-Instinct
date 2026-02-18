# Evolution of parental instict
HELLO ALL! Did you miss me? I'm sure you did, what with all the amazing and fun and totally unnecessary simulators I've been making recently =D. I'm back at it again, levelling up my skills in agent-based simulation by making the classic evoluionary biology simulator (and actually push to production this time). Inspired by the breadth of parental care behaviours in animals, I wanted to build a simulator to figure out what the "ideal" parental instict framework actually is. This one is considerably less practical than my other simulators, granted, as it won't be figuring out how to save lives or money. This one is for all the people like me who simply want to know "why?". What is the reason for nature's many implementations of the same basic principle: "Protect my child".

Let's be real here, as a tool, this is not going to be the most accurate evolutionary biology simulator ever created, but it's not going to be based on vibes either, don't worry. If you're interested in seeing which sources I used to come up with the rules for this simmulator, feel free to check them out:

[Parnetal Investment Contingency Model](https://www.sciencedirect.com/science/article/abs/pii/S109051380800007X)
[Heritability of Parental Effort in a Passerine Bird (2003)](https://pubmed.ncbi.nlm.nih.gov/14575340/)

But those were pretty long so the core TLDR is:
1. Parents' attentional resources were affected by both the amount of the resource and the reproductive potential of the child
2. Less Resources meant children with more reproductive potential received more of those resources. More resources meant the opposite.
3. The more parents invest in their children the more likely they are to survive (duh)
4. Parental effort is an inherited trait

Plus there was a fun I thought would be fun to add:
Adults try to find a mate, then if they can't (poor guys) they help raise the offspring of their siblings

And a boring one I thought would be run to remove (mwahaha):
Parental effort does not affect parent survival

Right I've yapped for a while, let's get into the project, shall we?

## Systeem architecture
```
        ---------------
        |    World    |
    ----|   --------  |-----------> Visualization
    |   | -|  Blip  | |
    |   | | --------  |-----------> Data Aggregation
    |   | |     ^     |                     ^
    |   ---------------                     |
    |     |     |                           |
    V     ------|----------------------------
   Move ---------

```