# Assigned-instance failure

Instance `i-0e2738df8255e6108` at `54.210.128.63` timed out on the first SSH
attempt. `aws ec2 describe-instances` immediately reported state
`shutting-down` (code 32). No source or output could be placed on or copied from
the instance. `aws ec2 wait instance-terminated` completed and a final describe
reported state `terminated` (code 48). The mandated explicit
`terminate-instances` call was then issued for this ID alone; AWS returned
current/previous state `terminated`, and the waiter plus final describe again
confirmed code 48. See `logs/aws-termination.log`. No other AWS instance or
resource was created, modified, or terminated.
