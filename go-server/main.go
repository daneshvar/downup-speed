package main

import (
	"fmt"
	"io"
	"log"
	"math/rand"
	"net"
	"os"
	"time"
)

func main() {
	arguments := os.Args
	if len(arguments) == 1 {
		fmt.Println("Please provide a port number!")
		return
	}

	PORT := ":" + arguments[1]

	log.Printf("Listen %s\n", PORT)
	l, err := net.Listen("tcp4", PORT)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer l.Close()
	rand.Seed(time.Now().Unix())

	for {
		c, err := l.Accept()
		if err != nil {
			fmt.Println(err)
			return
		}
		go handleConnection(c)
	}
}

func handleConnection(c net.Conn) {
	log.Printf("Serving %s\n", c.RemoteAddr().String())

	cmd := make([]byte, 1)
	for {
		n, err := c.Read(cmd)
		if err != nil {
			if err != io.EOF {
				log.Println("read error:", err)
			}
			break
		}

		if n > 0 {
			log.Printf("Command Received: %d\n", cmd)
			switch cmd[0] {
			case 2: // upload
				err = upload(c)
			case 3: // download
				err = download(c)
			}

			if err != nil {
				if err != io.EOF {
					log.Println("read error:", err)
				}
				break
			}
		} else {
			log.Println("read zero")
			break
		}
	}
	c.Close()
	log.Printf("Closed %s\n", c.RemoteAddr().String())
}

func upload(c net.Conn) (err error) {
	log.Println("Client Upload Start")

	buf := make([]byte, 1024*10) // big buffer
	total := 0
	for {
		n := 0
		n, err = c.Read(buf)
		if err != nil {
			break
		}

		if n > 0 {
			for i := range buf[:n] {
				if buf[i] == 0x04 { // finish
					log.Println("Client Upload Finished")
					log.Printf("Client Upload Total: %d\n", total)
					return
				}
			}
			total += n
		} else {
			err = fmt.Errorf("Read Zero")
			break
		}
	}
	log.Printf("Client Upload Total: %d\n", total)

	return
}

func download(c net.Conn) (err error) {
	log.Println("Client Download Start")
	buf := make([]byte, 1024*100) // big buffer
	total := 0
	for {
		n := 0
		n, err = c.Write(buf)
		if err != nil {
			cmd := make([]byte, 1)
			n, err2 := c.Read(cmd)
			if err2 != nil {
				break
			}

			if n > 0 {
				log.Printf("Command Received: %d\n", cmd)
				if cmd[0] == 0x04 {
					log.Println("Client Download Finish")
					err = nil
					break
				}
			}
			break
		}
		if n <= 0 {
			err = fmt.Errorf("Write Zero")
			break
		}
		total += n
	}

	log.Printf("Client Download Total: %d\n", total)

	return
}
