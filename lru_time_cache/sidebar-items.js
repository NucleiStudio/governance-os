initSidebarItems({"enum":[["Entry","A view into a single entry in an LRU cache, which may either be vacant or occupied."],["TimedEntry","Entry produced by `NotifyIter` that might be still valid or expired."]],"struct":[["Iter","An iterator over an `LruCache`'s entries that updates the timestamps as values are traversed. Values are produced in the most recently used order."],["LruCache","Implementation of LRU cache."],["NotifyIter","Much like `Iter` except will produce expired entries too where `Iter` silently drops them."],["OccupiedEntry","An occupied Entry."],["PeekIter","An iterator over an `LruCache`'s entries that does not modify the timestamp."],["VacantEntry","A vacant Entry."]]});