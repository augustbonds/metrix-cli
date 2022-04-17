#UDisc → DiscGolfMetrix Scorecard Migrator
This application will upload your UDisc Scorecards (.csv) to discgolfmetrix.com

It's a work in progress, many things are still hardcoded and it's only covering a few of Åland islands' courses.

#Usage
## Password file
Create a file with two lines containing your metrix username and password

'''
myuser@name.com
mysuperdupersecretpassword1234567
'''

Also download your scorecards as .csv (scorecards > sidebar > export to csv in the UDisc app)

Run the executable with two arguments: path to the password file and path to the .csv file.

#Caution
The program will upload only rounds with the player specified in main.rs. There is no automatic player lookup. It will insert rounds as the player logged in had played them.
It currently also hard-codes the current course to look insert, and doesn't consider alternative layouts.

