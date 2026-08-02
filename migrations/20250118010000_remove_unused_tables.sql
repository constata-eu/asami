DROP TABLE campaign_request_topics;                  
DROP TABLE campaign_requests;                        
DROP TABLE claim_account_requests;                   
DROP TABLE collab_requests;                          
DROP TABLE handle_request_topics;                    
DROP TABLE ig_campaign_rules;                        
DROP TABLE ig_crawl_results;                         
DROP TABLE ig_crawls;                                
DROP TABLE old_campaign_preferences;                 
DROP TABLE old_campaign_topics;                      
DROP TABLE old_collabs;                              
DROP TABLE old_handle_topics;                        
DROP TABLE old_ig_campaign_rules;                    
DROP TABLE old_topic_requests;                       
DROP TABLE old_campaigns;                            
DROP TABLE set_price_requests;                       
DROP TABLE set_score_and_topics_request_topics;      
DROP TABLE old_topics;                               
DROP TABLE set_score_and_topics_requests;            
DROP TABLE old_handles;                              

-- Delete all facebook auth methods.
DELETE from sessions WHERE auth_method_id IN (SELECT id FROM auth_methods WHERE kind = 'facebook');
DELETE from auth_methods WHERE kind = 'facebook';

-- Delete all handles.
DELETE FROM handle_topics WHERE handle_id in (SELECT id FROM handles WHERE site = 'instagram');
DELETE FROM handles WHERE site = 'instagram';

-- Remove handle alltogether.
ALTER TABLE handles DROP COLUMN site;
ALTER TABLE handles DROP COLUMN old_price;
ALTER TABLE handles DROP COLUMN old_status;
ALTER TABLE handles DROP COLUMN old_on_chain_tx_id;
ALTER TABLE handles DROP COLUMN legacy_score;

-- Only X campaigns.
ALTER TABLE campaigns DROP COLUMN campaign_kind;


