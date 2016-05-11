echo Creating tmp directory...

mkdir tmp
cd tmp

echo Adding files...

echo build > .gitignore
echo ^<html^>^</html^> > index.html
mkdir css
echo // main css file > css/main.css

echo Creating git repo with files...

call git init > NUL
call git config user.name "Test Script"
call git config user.email "lucio.franco@du.edu"
call git config core.autocrlf true

call git add --all
call git commit -m "inital commit" > NUL

echo Creating build folder and files...

mkdir build
echo // a bunch of js bs > build/bundle.js
echo # bash file > build/run.sh

echo Running command "ghp build"...
..\..\target\debug\ghp build -b test-branch

call git branch -a
call git status

call :sleep 2
rmdir /s /q build

echo Checking out "test-branch"...
call git checkout test-branch

echo Verify branch:
call :check_branch

cd ..
rmdir /s /q tmp

exit 0

:check_branch
if exist bundle.js echo Passed test #1 && exit /b 0
echo Failed test #1: could not find bundle.js in gh-pages branch
exit 1

:sleep
ping -n %1 127.0.0.1 > NUL
exit /b 0
