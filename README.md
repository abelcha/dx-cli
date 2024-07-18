![output](https://github.com/abelcha/dx-cli/assets/6186996/9f5f01de-dae6-4e02-a706-15c24c3fffa3)

How can finder show instant directory sizes, but dev tools takes 30 sec to walk though the file tree every time ?
Its ovbious macos caches dir sizes somewhere but Apple dont provide any api or document this feature

![Screenshot 2024-07-18 at 01 33 56](https://github.com/user-attachments/assets/bf51a21e-171c-4870-a12c-1220715018c8)

Its available throught a AppleScript's getinfo methods, but it is slow and unreliable
- libfffs (fast finder folder size) is an atempt to reverse engineer the underlying system call
- dx-cli is a wrapper around libfffs to provide a `du` type interface (and rust FFI bindings)



it provide 3 strategies that will fallback in this order by default:
- aev (AppleEvents https://en.wikipedia.org/wiki/Apple_events), a long forgotten IPC protocol using C with Pascal strings, works well but sometimes fails on concurent calls
- dstore .ds_store are a binary dumps of finder's internal database, finder regenerate it even if the call fails, so the combination of the two is surprisingly reliable. 
few parser attempts have been made since the OG that first reverse engineered it in perl, but none worked for every  around nested windows placement metadata;
i made a simpler implementation that only focus on 1st level size and dates, and works on every .ds_store i found on github code search
- live
fallback to classic recursive walkthrough of the file tree, safe and reliable but slow


