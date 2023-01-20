
echo "Enter your message"
read message
#echo 'Enter the name of the branch:'
#read branch
#

#if [ -n "$(git status - porcelain)" ];
#then
# echo "IT IS CLEAN"
#else
#

echo "git added all"

git add --all && git commit -m "${message}"


echo "Pushing data to remote server!!!"

git push -u origin main

echo "Pushed data successfully"

git status
