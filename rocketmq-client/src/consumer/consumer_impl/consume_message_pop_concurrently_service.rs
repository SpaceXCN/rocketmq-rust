/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::sync::Arc;

use rocketmq_common::common::message::message_client_ext::MessageClientExt;
use rocketmq_common::common::message::message_ext::MessageExt;
use rocketmq_common::common::message::message_queue::MessageQueue;
use rocketmq_common::ArcRefCellWrapper;
use rocketmq_common::WeakCellWrapper;
use rocketmq_remoting::protocol::body::consume_message_directly_result::ConsumeMessageDirectlyResult;

use crate::base::client_config::ClientConfig;
use crate::consumer::consumer_impl::consume_message_service::ConsumeMessageServiceTrait;
use crate::consumer::consumer_impl::default_mq_push_consumer_impl::DefaultMQPushConsumerImpl;
use crate::consumer::consumer_impl::pop_process_queue::PopProcessQueue;
use crate::consumer::consumer_impl::process_queue::ProcessQueue;
use crate::consumer::default_mq_push_consumer::ConsumerConfig;
use crate::consumer::listener::message_listener_concurrently::ArcBoxMessageListenerConcurrently;

pub struct ConsumeMessagePopConcurrentlyService {
    pub(crate) default_mqpush_consumer_impl: Option<WeakCellWrapper<DefaultMQPushConsumerImpl>>,
    pub(crate) client_config: ArcRefCellWrapper<ClientConfig>,
    pub(crate) consumer_config: ArcRefCellWrapper<ConsumerConfig>,
    pub(crate) consumer_group: Arc<String>,
    pub(crate) message_listener: ArcBoxMessageListenerConcurrently,
}

impl ConsumeMessagePopConcurrentlyService {
    pub fn new(
        client_config: ArcRefCellWrapper<ClientConfig>,
        consumer_config: ArcRefCellWrapper<ConsumerConfig>,
        consumer_group: String,
        message_listener: ArcBoxMessageListenerConcurrently,
    ) -> Self {
        Self {
            default_mqpush_consumer_impl: None,
            client_config,
            consumer_config,
            consumer_group: Arc::new(consumer_group),
            message_listener,
        }
    }
}

impl ConsumeMessageServiceTrait for ConsumeMessagePopConcurrentlyService {
    fn start(&mut self) {
        // nothing to do
    }

    fn shutdown(&self, await_terminate_millis: u64) {
        todo!()
    }

    fn update_core_pool_size(&self, core_pool_size: usize) {
        todo!()
    }

    fn inc_core_pool_size(&self) {
        todo!()
    }

    fn dec_core_pool_size(&self) {
        todo!()
    }

    fn get_core_pool_size(&self) -> usize {
        todo!()
    }

    async fn consume_message_directly(
        &self,
        msg: &MessageExt,
        broker_name: &str,
    ) -> ConsumeMessageDirectlyResult {
        todo!()
    }

    async fn submit_consume_request(
        &self,
        msgs: Vec<ArcRefCellWrapper<MessageClientExt>>,
        process_queue: Arc<ProcessQueue>,
        message_queue: MessageQueue,
        dispatch_to_consume: bool,
    ) {
        todo!()
    }

    async fn submit_pop_consume_request(
        &self,
        msgs: Vec<MessageExt>,
        process_queue: &PopProcessQueue,
        message_queue: &MessageQueue,
    ) {
        todo!()
    }
}