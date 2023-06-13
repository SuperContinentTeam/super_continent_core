#include<iostream>
#include<cstring>

#ifdef _WIN32
#include<winsock2.h>
#else
#include <sys/socket.h>
#include <netinet/in.h>
#include <sys/types.h>
#include <netdb.h>
#include <errno.h>
#include <fcntl.h>
#include <unistd.h>
typedef int SOCKET;
#endif

int main() {
#ifdef _WIN32
    WSADATA wsd;
    if(WSAStartup(MAKEWORD(2, 2), &wsd)){
        std::cout << "WSAStartup Error" << std::endl;
        return -1;
    }
#endif

    SOCKET tcpSocket = socket(AF_INET, SOCK_STREAM, 0);
    sockaddr_in sain{
        AF_INET,
        55555,
    };
    sain.sin_addr.S_un.S_addr = htonl(INADDR_ANY);

    if(bind(tcpSocket, (sockaddr *)&sain, sizeof(sockaddr)) == -1) {
        std::cout << "Bind Error" << std::endl;
        return -1;
    }

    listen(tcpSocket, 10);
    std::cout << "Listening 0.0.0.0:55555 ......" << std::endl;
    sockaddr clientAddr {AF_INET};

#ifndef _WIN32
    unsigned
#endif
    int clientAddrLen = sizeof(sockaddr);
    SOCKET linkSocket = accept(tcpSocket, &clientAddr, &clientAddrLen);
    std::cout << linkSocket << " 已连接" << std::endl;
    char recvBuf[1024] = {0};
    char sendBuf[1024] = "Hello world";
    while(true) {
        int ret = recv(linkSocket, recvBuf, 1024, 0);
        if (ret > 0) {
            std::cout << "接收数据: " << recvBuf << std::endl;
            memset(recvBuf, 0, 1024);
            send(linkSocket, sendBuf, (int)strlen(sendBuf), 0);
        }else if (ret == 0) {
            std::cout << "客户端关闭连接" << std::endl;
            break;
        }else{
            std::cout << "Error" << std::endl;
        }
    }

    #ifdef _WIN32
    closesocket(tcpSocket);
    closesocket(linkSocket);
    WSACleanup();
    #else
    close(tcpSocket);
    close(linkSocket);
    #endif

    std::cout << "已停止服务" << std::endl;

    return 0;
}
