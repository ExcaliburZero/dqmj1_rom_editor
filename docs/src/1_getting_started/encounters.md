# Modifying encounters
The **encounters table** is a data file within the game that details all of the enemies, bosses, starter monsters, and gift monsters. This table has one row for each such monster.

Using the Encounters tab, you can edit the stats, moves, skill sets, and more for each individual encounter.

<p align="center">
<img src="../images/encounters.png" alt="Encounters tab showing one of the starter monsters" style="max-height: 600px;" />
</p>

## Modifying a starter
As an example, lets change one of the starter monsters. The three starter monsters are in the table rows `048`, `049`, and `050`.

Let's edit encounter `048`, the starter Dracky.

Using the Species dropdown, we can change it from a Dracky to a Darkonium Slime. With the dropdown selected, you can start typing `darkonium` and `darkonium slime (28)` should be highlighted.

We can also give it a third skill set by clicking on the `Skill set (3)` dropdown and typing in `Wulfspade` to give it the Wulfspade skill set.

<p align="center">
<img src="../images/encounters_edited.png" alt="" style="max-height: 600px;" />
</p>

Now you can export the ROM with this change to the starter Dracky by clicking on the `Export patched ROM` button (or pressing `Ctrl + e`). Once you play up to point you get to choose your starter monster, you'll find the Darkonium Slime we created.

<p align="center">
<img src="../images/encounters_edited_ingame.png" alt="" style="max-height: 600px;" />
</p>

It's also a good idea to save the changes we made to the mod. To do that click on the `Save mod` button (or press `Ctrl + s`).

## Modifying other encounters
To modify an encounter, you'll need to find its encounter id.

As a general rule of thumb:

| Category | Encounter ID range |
|----------|----------|
| Bosses | 1 - 39 |
| Starters & Gift Monsters | 44 - 74 |
| Scout/Rival Monsters | 304 - 378, 431 - 751 |

To find a particular encounter, a good first place to look is the [DQM:J Wiki](https://dqmj.fandom.com/wiki/Dragon_Quest_Monsters:_Joker_Wiki).

For example, on the [Slime](https://dqmj.fandom.com/wiki/Slime) page you can find an encounter listing for the slimes that appear on Infant Isle. The codes in the bottom right corner are encounter ids in hexadecimal (`050`, `109`, and `10a`), and you can [convert them to decimal](https://www.rapidtables.com/convert/number/hex-to-decimal.html) (`80`, `265`, and `266`) to find the encounter ids to select and modify.

<p align="center">
<img src="../images/dqmj_wiki_slime.png" alt="" style="max-height: 600px;" />
</p>

Similar to the previous dropdowns, you can click on the encounter select dropdown (in the top middle of the page) and type in the encounter id (ex. `080`) to select the encounter to modify.

<p align="center">
<img src="../images/encounters_edit_slime_before.png" alt="" style="max-height: 600px;" />
</p>