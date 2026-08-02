-- Asami now publishes operational notifications through the English X account only.
-- Preserve historical sent rows, but stop pending Spanish jobs from retrying.
UPDATE tweet_queue
SET status = 'cancelled',
    last_error = 'spanish_notifications_retired',
    updated_at = now()
WHERE account_key = 'asami_spanish'
  AND status NOT IN ('sent', 'cancelled');
