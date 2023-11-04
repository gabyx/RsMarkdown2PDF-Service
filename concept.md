# How stuff works


Storage bucket > job-id.zip


Web (client) > API  > Blob (Disk) > uuid.zip
					> DB (Job) > uuid
					> Queue (Job) > uuid


					Queue > Service (md -> PDF) > Job > Blob > Vooodooo > PDF



Queue in / out