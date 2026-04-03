use std::collections::{BTreeMap, BTreeSet};

use petgraph::graph::DiGraph;

use crate::dqmj1_rom::events::{binary::InstructionOffset, disassembly::DecodedInstruction};

#[derive(Debug)]
pub struct CfgBlock<'a> {
    pub instructions: Vec<DecodedInstruction<'a>>,
}

#[derive(Debug)]
pub struct ControlFlowGraph<'a> {
    pub blocks: BTreeMap<InstructionOffset, CfgBlock<'a>>,
    pub graph: DiGraph<InstructionOffset, InstructionOffset>,
}

impl ControlFlowGraph<'_> {
    pub fn from_instructions<'a>(
        instructions: &'a BTreeMap<InstructionOffset, DecodedInstruction<'a>>,
    ) -> ControlFlowGraph<'a> {
        assert!(!instructions.is_empty());

        // Find leading instruction for each block
        let mut leaders = BTreeSet::<InstructionOffset>::new();
        leaders.insert(*instructions.iter().next().unwrap().0);

        for (offset, instruction) in instructions.iter() {
            let destinations = instruction.get_destinations(*offset);
            if let Some(jump_dest) = destinations.jump {
                leaders.insert(jump_dest);
                if let Some(normal_dest) = destinations.normal {
                    leaders.insert(normal_dest);
                }
            }
        }

        // Build up the blocks and record their relationships
        let mut blocks = BTreeMap::new();
        let mut edges = BTreeMap::new();
        for leader_offset in leaders.iter() {
            println!("leader_offset: {}", leader_offset);
            if let Some(leader) = instructions.get(leader_offset) {
                // Find the remaining instructions in the block
                let mut block_instructions = vec![leader.clone()];
                //let mut offset = leader.next_offset(*leader_offset);
                let mut offset = *leader_offset;
                let mut destinations = vec![];
                while let Some(instruction) = instructions.get(&offset) {
                    block_instructions.push(instruction.clone());

                    let next_destinations = instruction.get_destinations(offset);
                    println!("{:?} -> {:?}", instruction, next_destinations);

                    if let Some(jump_dest) = next_destinations.jump {
                        destinations.clear();

                        if let Some(normal_dest) = next_destinations.normal {
                            destinations.push(normal_dest);
                        }
                        destinations.push(jump_dest);

                        break;
                    }

                    offset = next_destinations.normal.unwrap();

                    destinations.clear();
                    destinations.push(offset);

                    // TODO: What about Exit?
                }

                blocks.insert(
                    *leader_offset,
                    CfgBlock {
                        instructions: block_instructions,
                    },
                );

                println!("{} -> {:?}", leader_offset, destinations);
                for dest in destinations {
                    if instructions.contains_key(&dest) {
                        edges.insert(*leader_offset, dest);
                    }
                }
            } else {
                panic!("Could not find leader instruction");
            }
        }

        // Convert into a graph
        let mut graph = DiGraph::new();
        let mut node_indices = BTreeMap::new();
        for (offset, _) in blocks.iter() {
            node_indices.insert(*offset, graph.add_node(*offset));
        }

        for (src, dest) in edges.iter() {
            if !node_indices.contains_key(src) {
                println!("Failed to find node for index: {}", src);
            }
            if !node_indices.contains_key(dest) {
                println!("Failed to find node for index: {}", dest);
            }

            graph.add_edge(
                *node_indices.get(src).unwrap(),
                *node_indices.get(dest).unwrap(),
                1,
            );
        }

        ControlFlowGraph { blocks, graph }
    }
}
