# Recipes to Groceries

A program to make a grocery list out of your saved recipes. Recipes can be scraped from websites that support JSON LD, 
or are otherwise entered manually. Change how much of each recipe you want to make and receive a grocery list 
that sorts items by their aisle

The full list of features available are listed below.

## Features

- [ ] Recipe database
  - [x] Scrapes recipes from websites using the [recipe schema](https://schema.org/Recipe) and [JSON LD](https://json-ld.org/)
  - [ ] Manual submission of recipe form and editing of scraped recipe data
  - [ ] Stores recipes in local SQLite DB
- [ ] Create shopping lists
  - [ ] Select saved recipes to compile a grocery list
    - [ ] Combines identical ingredients
    - [ ] Gives option to combine similar ingredients
  - [ ] Modify number of servings to purchase for each recipe
  - [ ] Scrapes aisle and price information from local:
    - [X] Walmart
    - [ ] Ralphs
  - [X] Finds similar products in store, if original ingredients weren't available
- [ ] Recommend recipes using extra ingredients not fully used by recipes