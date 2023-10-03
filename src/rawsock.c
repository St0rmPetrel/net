#include <sys/socket.h>
#include <unistd.h>
#include <netinet/ip_icmp.h>
#include <errno.h>
#include <string.h>

extern int errno;

int get_rs(int ttl, int rcv_timeout_sec) {
  struct timeval tv;

  int sockfd = socket(AF_INET, SOCK_RAW, IPPROTO_ICMP);
  if(sockfd<0) {
    return (-1);
  }
  // set IP ttl
  if (setsockopt(
        sockfd, IPPROTO_IP,
        IP_TTL, &ttl, sizeof(ttl)
      ) != 0) {
        return (-2);
  }

  tv.tv_sec = rcv_timeout_sec;
  tv.tv_usec = 0;
  // set rcv timeout
  if (setsockopt(
        sockfd, SOL_SOCKET,
        SO_RCVTIMEO, (const char*)&tv, sizeof tv
      ) != 0) {
    return (-3);
  }
  return sockfd;
}

char* get_err_dscr() {
  return strerror(errno);
}

int send_rs() {
  return 0;
}

int resv_rs() {
  return 0;
}

int close_rs(int sock) {
  return close(sock);
} 
