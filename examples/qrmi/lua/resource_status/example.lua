--
-- (C) Copyright IBM 2026
--
-- This code is licensed under the Apache License, Version 2.0. You may
-- obtain a copy of this license in the LICENSE.txt file in the root directory
-- of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.
--
-- Any modifications or derivative works of this code must retain this
-- copyright notice, and modified files need to carry a notice indicating
-- that they have been altered from the originals.
--
package.cpath = package.cpath .. ";./?.so"
local qrmi = require("qrmi")

if #arg ~= 2 then
    print("Missing arguments\n")
    print("Usage: lua example.lua <resource_type> <resource_id>>\n")
    os.exit(1)
end

-- Create a resource handle (corresponds to the real qrmi_resource_new)
local resource, err = qrmi.new(arg[2], arg[1])
if not resource then
    print("new failed:", err)
    os.exit(1)
end
print("resource created")

local rstatus, rstatus_err = resource:status()
if not rstatus then
    print("status failed:", rstatus_err)
else
    print("status:")
    print("  status            =", rstatus.status)
    print("  status_reason     =", rstatus.status_reason)
    print("  healthy           =", rstatus.healthy)
    print("  pending_job_count =", rstatus.pending_job_count)
    print("  is_accessible     =", rstatus.is_accessible)
    if rstatus.capacity then
        print("  capacity.available_slots =", rstatus.capacity.available_slots)
        print("  capacity.max_slots       =", rstatus.capacity.max_slots)
    else
        print("  capacity          = nil")
    end
end

resource:free()
print("resource freed")
