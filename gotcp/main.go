package main

import (
	"bufio"
	"fmt"
	"log"
	"net"
	"strings"
)

var history = []Message{}
var clients []Client

type Client struct {
	conn     net.Conn
	username string
}

type Message struct {
	sender  Client
	content string
}

func main() {
	// Create a new TCP server
	listener, err := net.Listen("tcp", ":8080")
	if err != nil {
		// Panic if something went wrong
		panic(err)
	}

	// Close the listener when the main function ends
	defer listener.Close()

	fmt.Println("Chat server started on port 8080")

	for {
		// Accept a new connection
		conn, err := listener.Accept()
		if err != nil {
			log.Println(err)
			continue
		}

		// Handle the connection in a new goroutine
		go handleClient(conn)
	}
}

func handleClient(conn net.Conn) {
	// Create a new client
	// and add it to the list of clients
	client := Client{conn: conn}

	client.username = getUsername(conn)
	clients = append(clients, client)

	receiveMessages(client)
}

func getUsername(conn net.Conn) string {
	reader := bufio.NewReader(conn)
	conn.Write([]byte("Enter your username: "))
	username, _ := reader.ReadString('\n')

	username = strings.TrimSpace(username)

	does_exist := Find(clients, func(c Client) bool {
		return c.username == username
	})

	if does_exist {
		conn.Write([]byte("An error occurred. Please try again.\n"))
		return getUsername(conn)
	}

	conn.Write([]byte(fmt.Sprintf("Welcome, %s!\n", username)))
	return username
}

func receiveMessages(client Client) {
	// Send the chat history to the client
	for _, message := range history {
		client.conn.Write([]byte(fmt.Sprintf("[%s]: %s\n", message.sender.username, message.content)))
	}
	reader := bufio.NewReader(client.conn)

	for {
		// Read the message from the client
		message_raw, err := reader.ReadString('\n')
		if err != nil {
			if err.Error() == "EOF" {
				fmt.Printf("Client '%s' disconnected\n", client.username)
			} else {
				fmt.Println(err)
			}
			break
		}

		message_raw = strings.TrimSpace(message_raw)

		if message_raw != "" {
			message := Message{sender: client, content: message_raw}
			history = append(history, message)
			sendMessageToClients(message)
		}
	}

	client.conn.Close()
	clients = Remove(clients, client)
}

func sendMessageToClients(message Message) {
	for _, client := range clients {
		if client.username != message.sender.username {
			client.conn.Write([]byte(fmt.Sprintf("[%s]: %s\n", message.sender.username, message.content)))
		}
	}
}

func Find[T any](items []T, predicate func(T) bool) bool {
	for _, item := range items {
		if predicate(item) {
			return true
		}
	}
	return false
}

func Remove[T comparable](items []T, item T) []T {
	for i, v := range items {
		if v == item {
			return append(items[:i], items[i+1:]...)
		}
	}
	return items
}
