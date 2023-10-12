#include <sys/socket.h>
#include <unistd.h>
#include <netinet/ip_icmp.h>

#define GET_RS_FAIL_CREATE_SOCK -1
#define GET_RS_FAIL_SET_TTL -2
#define GET_RS_FAIL_SET_TIMOUT -3

// get_icmp_rs opens a ip-icmp socket.
int get_icmp_rs(int ttl, int rcv_timeout_sec) {
  struct timeval tv;

  int sockfd = socket(AF_INET, SOCK_RAW, IPPROTO_ICMP);
  if(sockfd<0) {
    return (GET_RS_FAIL_CREATE_SOCK);
  }
  // set IP ttl
  if (setsockopt(
        sockfd, IPPROTO_IP,
        IP_TTL, &ttl, sizeof(ttl)
      ) != 0) {
        return (GET_RS_FAIL_SET_TTL);
  }

  tv.tv_sec = rcv_timeout_sec;
  tv.tv_usec = 0;
  // set rcv timeout
  if (setsockopt(
        sockfd, SOL_SOCKET,
        SO_RCVTIMEO, (const char*)&tv, sizeof tv
      ) != 0) {
    return (GET_RS_FAIL_SET_TIMOUT);
  }
  return sockfd;
}
