#### background

My name is James. My (not so anonymous) pseudonym is sanjams.

I like to consider myself an engineer; I'm interested in understanding how things work and solving problems of all shapes and sizes. Today, the form of engineering I mostly practice is software engineering. My focus here has mostly been on backend distributed systems, but I have dabbled in many other areas of the stack as well (the lower the better!). I tend to work quickly and ask questions later.

I am originally from Atlanta. I have lived in Seattle and now reside in New York City.

I studied Industrial Engineering and Computer Science at the Georgia Institute of Technology with a focus in Quality and Statistics and Networking respectively. I graduated in 2017.

I worked at AWS from 2017 to June 2025. Within AWS, I spent the majority of my time (~6 years) working on [AWS SageMaker](https://aws.amazon.com/sagemaker-ai/). I spent about four years on the Inference team primarily focusing on the real-time inference system which was responsible for handling millions of requests per second. Some of my projects included [Model I/O Capture](https://docs.aws.amazon.com/sagemaker/latest/dg/model-monitor-data-capture-endpoint.html), [Asynchronous Inference](https://docs.aws.amazon.com/sagemaker/latest/dg/async-inference.html), [Serverless Inference](https://docs.aws.amazon.com/sagemaker/latest/dg/serverless-endpoints.html), [Model Response Streaming](https://aws.amazon.com/blogs/machine-learning/elevating-the-generative-ai-experience-introducing-streaming-support-in-amazon-sagemaker-hosting/), and performance optimizations for our data-plane reducing latency overhead per request to single-digit milliseconds. I then spent about two years on the Training team where I primarily focused on building the HyperPod compute platform. My projects included [Launching HyperPod](https://aws.amazon.com/blogs/aws/introducing-amazon-sagemaker-hyperpod-a-purpose-built-infrastructure-for-distributed-training-at-scale/), [Kubernetes support for HyperPod](https://aws.amazon.com/blogs/machine-learning/introducing-amazon-eks-support-in-amazon-sagemaker-hyperpod/), and improving our fault recovery system for [Nova Model Pre-Training](https://www.aboutamazon.com/news/aws/amazon-nova-artificial-intelligence-bedrock-aws) ([where we achieved 97% goodput!](https://arxiv.org/pdf/2506.12103v1)). I greatly enjoyed my time at AWS; I am now onto something new...

While I have built new things, I generally prefer improving things that already exist.

#### projects

- [emf validator](https://emfvalidator.com) - simple website for validating AWS EMF logs directly in your browser
- [sfn-profiler](https://github.com/sanjams2/sfn-profiler) - tracing/profiling tools for AWS Step Functions
- [tiny-analytics](https://github.com/sanjams2/tiny-analytics) - worlds simplest metrics framework; emit metrics from anywhere with no SDK!
- [govpipe](https://govpipe.com) - do you like permits? neither do we
- [libretorrent](http://libretorrent.com) - goodwill for books

#### ideas (free!)

Hoping to eventually move these to the project section above, but feel free to take it on yourself

- Distributed Training Visualizer - a visualization tool that takes model code, model config, and infra config and renders how the job would be mapped across accelerators (using WebGPU!)
- Distributed Training Bandwidth Calculator - a tool for calculating the bandwidth requirements between different accelerators used within a model training run (likely built on top of/with visualizer above)
- Hierarchical All-Reduce - mostly a learning exercise to build a framework for performing all-reduce in a hierarchical fashion
- Claude Code Conversation Explorer - a tool (likely within Claude Code itself) which visualizes Claude Code conversations
- SRE Battle RL Environment - building an RL environment where an agent learning to be an SRE faces off against a ["chaos-monkey"](https://netflix.github.io/chaosmonkey/) agent. Reward criteria roughly based on alarms in OK state.
- DSAAS - this ones for me...

#### profiles

- [github](https://github.com/sanjams2)
- [twitter](https://twitter.com/james3sanders)
