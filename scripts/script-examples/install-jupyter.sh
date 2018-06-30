# sudo apt-get update
# sudo apt-get install -y python3-pip
# pip3 install jupyter
killall jupyter-notebook
sleep 1
nohup /home/ubuntu/.local/bin/jupyter notebook --no-browser --port=8888 < /dev/null > /dev/null 2>&1 &
sleep 2

echo
echo "The Jupyter Notebook is running on the cluster at the address below."
echo "To access it from your computer, do as follows:"
echo
echo "1. Get the IP address of the cluster"
echo
echo "  tail -1 ./neighbors.txt && echo"
echo
echo "2. Find the path to the EC2 key that used for launching the cluster."
echo "If your setup is using the credential file, it is likely that you can find the path to" \
"your EC2 key by running the following command:"
echo
echo "  cat \$(cat ~/.tmsn_config | grep credential | awk '{print \$NF}') | grep ssh_key | awk '{print \$NF}'"
echo
echo "3. Forward the port of Jupyter notebook on the remote server to your computer"
echo
echo "  ssh -i <path to your EC2 key> -L 8888:localhost:8888 ubuntu@<the cluster ip>"
echo
echo "4. Keep the ssh command above running in background, then open the following address using" \
"the browser on your computer."
echo
echo "  http"$(/home/ubuntu/.local/bin/jupyter notebook list | grep -Po '(?<=http).*(?=::)')
echo