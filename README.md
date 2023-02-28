# rust-new-project-template
A good starting point for a new Rust project

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)

## Introduction
In project 2, the purpose is to build a functional Web Microservice in Rust and deliver it on Cloud based Container Registery (ECR). I create a simple actix Microservice for checking stock current price, high price, low price and open price that you are interested in.

This actix Microservice has multiple routes:

A. type: "/" that returns a message : 
        <p>you can get close price of stock you are interested in</p>
        <p>you can get high prices of stock you are interested in</p>
        <p>you can get low prices of stock you are interested in</p> 
        <p>you can get open prices of stock you are interested in</p>
![image](https://user-images.githubusercontent.com/122952572/222003813-f91c751c-b8f0-4e7b-876a-6b2abcfd1763.png)

        

B. type: "/openprice/{name}" that returns a open pirce for the stock you type
`/openprice/TSLA`
![image](https://user-images.githubusercontent.com/122952572/222004340-99c775e1-8d0b-4c92-b790-5b5f4c4a8a30.png)



C. type: "/lowprice/{name}" that returns a low pirce for the stock you type
`/lowprice/TSLA`
![image](https://user-images.githubusercontent.com/122952572/221999515-f0d96233-6fa1-4ba8-895b-f8a67c89fb1d.png)


D. type: "/highprice/{name}" that returns a high pirce for the stock you type
`/highprice/TSLA`
![image](https://user-images.githubusercontent.com/122952572/222004463-60433b32-e3f0-4af4-9faf-278a11a9bf60.png)


E. type: "/currprice/{name}" that returns a current pirce for the stock you type
`/currprice/TSLA`
![image](https://user-images.githubusercontent.com/122952572/222004529-5c159f74-5455-46ed-922f-9d449d1405fd.png)

## Use Cloud based Container Registery (ECR) to deploy the project
1. Go to AWS Cloud9, then click "Create environment"
2. install rust in Cloud9
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. git clone from your repository
`git clone https://github.com/ht175/IDS721-Project2.git`
4. create a docker file
`touch Dockerfile`
![image](https://user-images.githubusercontent.com/122952572/222005588-6a23728d-c0fd-4849-8f14-3a5621c014aa.png)
5.Go to AWS Amazon Elastic Container Registry, create a private repository and click view push command
follow the command
when try to build docker, if it failed beacuase of no space left 
run this command
`curl -s https://gist.githubusercontent.com/wongcyrus/a4e726b961260395efa7811cab0b4516/raw/6a045f51acb2338bb2149024a28621db2abfcaab/resize.sh | bash /dev/stdin 60
`
6.Go to AWS APP Runner
Click "create service" to set up configuration (choose the repository you created)

When finish, you can get

![image](https://user-images.githubusercontent.com/122952572/222006612-ec45c5fa-cbbb-4ce9-9f2b-6b254e574445.png)



