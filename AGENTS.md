# Metis

Implementation guide for working on Metis.

## LEGACY PLANS
Previous legacy plans and dev log can be found in ./plans and ./notes.

## What is Metis?

A Rust based system for orchestating personal AI Agents.

## Definitions
* Channel: a top level conversation with an AI agent
* Branch: a branch or sub-conversation within a channel
* Task: a unit of work or goal created by an AI agent in it's current run
* History: the conversation history for a channel, branch or worker
* Worker: an AI agent worker process allowing agents to run in the background
