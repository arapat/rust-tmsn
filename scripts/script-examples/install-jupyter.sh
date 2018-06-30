# sudo apt-get update
# sudo apt-get install -y python3-pip
# pip3 install jupyter
killall jupyter-notebook
sleep 1
nohup /home/ubuntu/.local/bin/jupyter notebook --no-browser --port=8888 < /dev/null > /dev/null 2>&1 &
sleep 2

echo
echo "The Jupyter Notebook is running on the cluster at the address below. To access it from your" \
"computer, forward the port on the cluster to a local port using some command like: "
echo
echo "  ssh -i <path to your EC2 key> -L 8888:localhost:8888 ubuntu@<cluster ip>"
echo
echo "The cluster IP can be found by running"
echo
echo "  tail -1 ./neighbors.txt && echo"
echo
echo "Keep the ssh command above running in background, then open the following address using" \
"the browseron your computer."
echo
echo "Jupyter Notebook URL:"
echo "  http"$(/home/ubuntu/.local/bin/jupyter notebook list | grep -Po '(?<=http).*(?=::)')