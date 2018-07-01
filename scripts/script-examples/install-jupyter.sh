sudo apt-get update
sudo apt-get install -y python3-pip
pip3 install jupyter
killall jupyter-notebook
sleep 1
nohup /home/ubuntu/.local/bin/jupyter notebook --no-browser --port=8888 < /dev/null > /dev/null 2>&1 &
sleep 2

echo
echo "The Jupyter Notebook is running on the cluster at the address below."
echo
echo "Open the following address using the browser on your computer"
echo
URL=$(dig +short myip.opendns.com @resolver1.opendns.com)
echo "  http"$(/home/ubuntu/.local/bin/jupyter notebook list | grep -Po '(?<=http).*(?=::)' | sed "s/localhost/$URL/")
echo