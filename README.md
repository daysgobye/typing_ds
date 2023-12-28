## Usage
```
.\.env\Scripts\Activate.ps1
python .\py\main.py
maturin develop

pyuic5 -x .\gui\typing_data.ui -o .\py\window_gui.py

$env:PYAPP_PROJECT_NAME	="bs_typing_data"
$env:PYAPP_PROJECT_VERSION	="0.0.1"
$env:PYAPP_PROJECT_DEPENDENCY_FILE	="C:\Users\Owner\Documents\code\typing_ds\requirements.txt"
$env:PYAPP_EXEC_SCRIPT	="C:\Users\Owner\Documents\code\typing_ds\py\main.py"
cargo install pyapp --root out
pyinstaller --onefile .\py\main.py



```
```bash
$ npm install # or pnpm install or yarn install
```


# goals

Build a set of tests then these tests can be run on any group of keylogs.
**tests:**
* least common bigrams trigram and quadgrams (good combos)
* most common words (ok value to be put on bigrams)
* most common Utterance / set of words (huge value)
* most used combos
* unused combos
* avrage typing speed
* peek typing speed

With this set of tests I want to do a few things

run the set of tests on click to run it over all keylogs

on a cron job run it over the new stuff.
so every x hours run tests over all the keylogs senc the last time it was run and save results and a time stamp of when it was run. 
this will let us track improvment and changes 

When asked it will run the tests over all keylogs this will be saved for review 
this I think is the best data because it has your largest sample size.


# I also want some real time stuff 
these are smaller tests that should run all the time on incoming utterances to provide feedback

**tests:**

* real time typing speed
* missed expantions 
* used expantions 

these will let us do things like:

* send a notification when you miss a combo 
* send a notification when you hit combo you have a high miss rate on 
* update widget with current typing speed 


# long shots 

build ai prompting into anything you want.
so in the real time part we lesson for "codeprompt:" then use the next bit of typing until ":end" as the prompt we can even do tings like swap out "CLIPBOARD" for your curent clipboard contents

we can have a few prompts avaiable for you to use too.
* codeprompt: for code 
* prompt: for genaric 
* email: for email 

these could be better then normal ai because we can feed it your wrightin as a refrance for style 

# combos

users can add there combos to the app for us to keep track of

combo:
letters: array of strinings
result: string

