function check_branch() {
    echo "Checking files..."
    
    if [ -f bundle.js ] && [ -f run.sh ]
    then
	echo "Passed test #1"
	return
    fi

    echo "Failed test #1: could not find bundle.js in gh-pages branch"
    exit 1
}
