ftp -n 192.168.1.76 2121 <<End-Of-Session
user anonymous a

binary
cd /mnt/ext1/applications


put my_app.app test_vk_1.app

bye
End-Of-Session