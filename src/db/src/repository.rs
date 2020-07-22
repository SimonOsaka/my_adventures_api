// use crate::models::{Article, NewArticle, NewComment, NewUser, UpdateUser};
// use crate::queries::{articles, comments, favorites, followers, users};
// use crate::shims::{to_article, to_comment};
use crate::queries::adventures;
// use crate::models::MyList;
use crate::DateTimeUtils;
use crate::Repo;
use anyhow::Error as OpaqueError;
use anyhow::Result;
use domain::DatabaseError;
// use diesel::result::{DatabaseErrorKind, Error};
// use domain;
// use domain::{DatabaseError, DeleteCommentError, GetUserError};
// use std::collections::HashSet;
// use uuid::Uuid;

// /// Helper function to cast a diesel::Error into a domain Database Error.
// /// This requires casting the diesel::Error into anyhow::Error first.
pub fn to_db_error(e: OpaqueError) -> domain::DatabaseError {
    domain::DatabaseError::from(e)
}

#[derive(Clone, Debug)]
pub struct Repository(pub Repo);

#[async_trait]
impl domain::repositories::Repository for Repository {
    async fn find_adventures(
        &self,
        query: domain::AdventuresQuery,
    ) -> Result<Vec<domain::Adventures>, DatabaseError> {
        let my_list_result;
        if query.item_id != 0 {
            my_list_result = adventures::find_by_item_type(&self.0, query).await;
        } else {
            my_list_result = adventures::find_latest(&self.0, query).await;
        }
        let result: Vec<domain::Adventures> = my_list_result
            .map_err(to_db_error)
            .unwrap()
            .into_iter()
            .map(|m| domain::Adventures {
                id: m.id,
                title: m.title,
                image_url: m.image_url,
                created_at: DateTimeUtils::beijing(m.created_at),
                item_type: m.item_type,
                link: m.link,
            })
            .collect();
        Ok(result)
    }

    async fn update_adventure(
        &self,
        update: domain::AdventuresUpdate,
    ) -> Result<bool, DatabaseError> {
        let updated: bool = adventures::update(&self.0, update.into())
            .await
            .map_err(to_db_error)
            .unwrap();

        Ok(updated)
    }

    async fn delete_adventure(&self, _id: u64) -> Result<bool, DatabaseError> {
        let deleted = adventures::delete(&self.0, _id).await?;

        Ok(deleted)
    }
    async fn insert_adventure(
        &self,
        draft: domain::AdventureContent,
    ) -> Result<u64, DatabaseError> {
        let id: u64 = adventures::insert(&self.0, draft.into()).await.unwrap();

        Ok(id)
    }

    async fn get_adventure_by_id(
        &self,
        id: u64,
    ) -> Result<Option<domain::Adventures>, DatabaseError> {
        let my = adventures::find_one(&self.0, id)
            .await
            .map_err(to_db_error)
            .unwrap();

        let result = match my {
            Some(ad) => Some(domain::Adventures {
                id: ad.id,
                title: ad.title,
                image_url: ad.image_url,
                created_at: DateTimeUtils::beijing(ad.created_at),
                item_type: ad.item_type,
                link: ad.link,
            }),
            _ => None,
        };

        Ok(result)
    }
    //     fn publish_article(
    //         &self,
    //         draft: domain::ArticleContent,
    //         author: &domain::User,
    //     ) -> Result<domain::Article, domain::PublishArticleError> {
    //         let result: Article = articles::insert(&self.0, NewArticle::from((&draft, author)))
    //             .map_err(|e| match e {
    //                 Error::DatabaseError(kind, _) => match kind {
    //                     DatabaseErrorKind::UniqueViolation => {
    //                         domain::PublishArticleError::DuplicatedSlug {
    //                             slug: draft.slug(),
    //                             source: to_db_error(e),
    //                         }
    //                     }
    //                     _ => to_db_error(e).into(),
    //                 },
    //                 e => to_db_error(e).into(),
    //             })?;
    //         let article = to_article(result, author.to_owned(), 0);
    //         Ok(article)
    //     }

    //     fn get_article_by_slug(&self, slug: &str) -> Result<domain::Article, domain::GetArticleError> {
    //         Ok(articles::find_one(&self.0, &slug).map_err(to_db_error)?)
    //     }

    //     fn get_article_view(
    //         &self,
    //         viewer: &domain::User,
    //         article: domain::Article,
    //     ) -> Result<domain::ArticleView, domain::GetArticleError> {
    //         let author_view = self
    //             .get_profile_view(viewer, &article.author.username)
    //             .unwrap();
    //         let is_favorite =
    //             favorites::is_favorite(&self.0, viewer.id, &article.slug).map_err(to_db_error)?;
    //         let article_view = domain::ArticleView {
    //             content: article.content,
    //             slug: article.slug,
    //             author: author_view,
    //             metadata: article.metadata,
    //             favorited: is_favorite,
    //             favorites_count: article.favorites_count,
    //             viewer: viewer.id,
    //         };
    //         Ok(article_view)
    //     }

    //     fn get_articles_views(
    //         &self,
    //         viewer: &domain::User,
    //         articles: Vec<domain::Article>,
    //     ) -> Result<Vec<domain::ArticleView>, DatabaseError> {
    //         let slugs: Vec<String> = articles.iter().map(|a| a.slug.to_owned()).collect();
    //         let slugs: Vec<&str> = slugs.iter().map(|slug| slug.as_str()).collect();

    //         let favs = favorites::are_favorite(&self.0, viewer.id, slugs).map_err(to_db_error)?;
    //         articles
    //             .into_iter()
    //             .map(|a| {
    //                 let favorited = favs[a.slug.as_str()];
    //                 let author_view = self.get_profile_view(viewer, &a.author.username)?;
    //                 let article_view = domain::ArticleView {
    //                     content: a.content,
    //                     slug: a.slug,
    //                     author: author_view,
    //                     metadata: a.metadata,
    //                     favorited,
    //                     favorites_count: a.favorites_count,
    //                     viewer: viewer.id,
    //                 };
    //                 Ok(article_view)
    //             })
    //             .collect()
    //     }

    //     fn find_articles(
    //         &self,
    //         query: domain::ArticleQuery,
    //     ) -> Result<Vec<domain::Article>, DatabaseError> {
    //         let result: Vec<domain::Article> = articles::find(&self.0, query)
    //             .map_err(to_db_error)?
    //             .into_iter()
    //             .map(|(a, u, n_fav)| {
    //                 let u: domain::User = u.into();
    //                 to_article(a, u, n_fav)
    //             })
    //             .collect();
    //         Ok(result)
    //     }

    //     fn feed(
    //         &self,
    //         user: &domain::User,
    //         query: domain::FeedQuery,
    //     ) -> Result<Vec<domain::ArticleView>, DatabaseError> {
    //         let articles: Vec<domain::Article> =
    //             articles::feed(&self.0, user.id, query.limit, query.offset)
    //                 .map_err(to_db_error)?
    //                 .into_iter()
    //                 .map(|(a, u, n_fav)| {
    //                     let u: domain::User = u.into();
    //                     to_article(a, u, n_fav)
    //                 })
    //                 .collect();
    //         Ok(self.get_articles_views(user, articles)?)
    //     }

    //     fn delete_article(&self, article: &domain::Article) -> Result<(), DatabaseError> {
    //         Ok(articles::delete(&self.0, &article.slug).map_err(to_db_error)?)
    //     }

    //     fn comment_article(
    //         &self,
    //         user: &domain::User,
    //         article: &domain::Article,
    //         comment: domain::CommentContent,
    //     ) -> Result<domain::Comment, DatabaseError> {
    //         let new_comment = NewComment {
    //             body: &comment.0,
    //             article_id: &article.slug,
    //             author_id: user.id,
    //         };
    //         let raw_comment = comments::create_comment(&self.0, new_comment).map_err(to_db_error)?;
    //         let comment = domain::Comment {
    //             id: raw_comment.id as u64,
    //             author: user.profile.clone(),
    //             body: raw_comment.body,
    //             created_at: raw_comment.created_at,
    //             updated_at: raw_comment.updated_at,
    //         };
    //         Ok(comment)
    //     }

    //     fn get_comment(&self, comment_id: u64) -> Result<domain::Comment, DeleteCommentError> {
    //         let comment = comments::get_comment(&self.0, comment_id).map_err(|e| match e {
    //             Error::NotFound => DeleteCommentError::CommentNotFound {
    //                 comment_id,
    //                 source: to_db_error(e),
    //             },
    //             e => to_db_error(e).into(),
    //         })?;
    //         let author = users::find(&self.0, comment.author_id).map_err(to_db_error)?;
    //         Ok(to_comment(comment, author))
    //     }

    //     fn get_comments(
    //         &self,
    //         article: &domain::Article,
    //     ) -> Result<Vec<domain::Comment>, DatabaseError> {
    //         let comments: Vec<_> = comments::get_comments(&self.0, &article.slug)
    //             .map_err(to_db_error)?
    //             .into_iter()
    //             .map(|(c, u)| to_comment(c, u))
    //             .collect();
    //         Ok(comments)
    //     }

    //     fn delete_comment(&self, comment_id: u64) -> Result<(), DeleteCommentError> {
    //         Ok(comments::delete_comment(&self.0, comment_id).map_err(to_db_error)?)
    //     }

    //     fn update_article(
    //         &self,
    //         article: domain::Article,
    //         update: domain::ArticleUpdate,
    //     ) -> Result<domain::Article, DatabaseError> {
    //         articles::update(&self.0, (&update).into(), &article.slug).map_err(to_db_error)?;
    //         let article = self.get_article_by_slug(&article.slug)?;
    //         Ok(article)
    //     }

    //     fn favorite(
    //         &self,
    //         article: &domain::Article,
    //         user: &domain::User,
    //     ) -> Result<domain::FavoriteOutcome, domain::DatabaseError> {
    //         Ok(favorites::favorite(&self.0, user.id, &article.slug).map_err(to_db_error)?)
    //     }

    //     fn unfavorite(
    //         &self,
    //         article: &domain::Article,
    //         user: &domain::User,
    //     ) -> Result<domain::UnfavoriteOutcome, domain::DatabaseError> {
    //         Ok(favorites::unfavorite(&self.0, user.id, &article.slug).map_err(to_db_error)?)
    //     }

    //     fn sign_up(&self, sign_up: domain::SignUp) -> Result<domain::User, domain::SignUpError> {
    //         let new_user = NewUser {
    //             username: &sign_up.username,
    //             email: &sign_up.email,
    //             password: sign_up.password.hash(),
    //             id: Uuid::new_v4(),
    //         };
    //         Ok(users::insert(&self.0, new_user)
    //             .map_err(to_db_error)?
    //             .into())
    //     }

    //     fn update_user(
    //         &self,
    //         user: domain::User,
    //         update: domain::UserUpdate,
    //     ) -> Result<domain::User, DatabaseError> {
    //         let update = UpdateUser::from(&update);
    //         let updated = users::update(&self.0, user.id, update).map_err(to_db_error)?;
    //         Ok(domain::User::from(updated))
    //     }

    //     fn get_user_by_id(&self, user_id: Uuid) -> Result<domain::User, GetUserError> {
    //         let result = users::find(&self.0, user_id);
    //         let user = result.map_err(|e| match e {
    //             e @ Error::NotFound => domain::GetUserError::NotFound {
    //                 user_id,
    //                 source: to_db_error(e),
    //             },
    //             e => to_db_error(e).into(),
    //         })?;
    //         Ok(domain::User::from(user))
    //     }

    //     fn get_user_by_email_and_password(
    //         &self,
    //         email: &str,
    //         password: &str,
    //     ) -> Result<domain::User, domain::LoginError> {
    //         let result = users::find_by_email(&self.0, email);
    //         let user = result.map_err(|e| match e {
    //             Error::NotFound => domain::LoginError::NotFound,
    //             e => to_db_error(e).into(),
    //         })?;

    //         // Check if the provided password is valid
    //         let stored_password = domain::Password::from_hash(user.password.to_owned());
    //         if !stored_password.verify(&password)? {
    //             return Err(domain::LoginError::NotFound);
    //         }

    //         Ok(domain::User::from(user))
    //     }

    //     fn get_profile(&self, username: &str) -> Result<domain::Profile, GetUserError> {
    //         let user = users::find_by_username(&self.0, username).map_err(to_db_error)?;
    //         Ok(domain::Profile::from(user))
    //     }

    //     fn get_profile_view(
    //         &self,
    //         viewer: &domain::User,
    //         username: &str,
    //     ) -> Result<domain::ProfileView, GetUserError> {
    //         let viewed_user = users::find_by_username(&self.0, username).map_err(to_db_error)?;
    //         let following =
    //             followers::is_following(&self.0, viewer.id, viewed_user.id).map_err(to_db_error)?;
    //         let view = domain::ProfileView {
    //             profile: domain::Profile::from(viewed_user),
    //             following,
    //             viewer: viewer.id,
    //         };
    //         Ok(view)
    //     }

    //     fn follow(
    //         &self,
    //         follower: &domain::User,
    //         to_be_followed: &domain::Profile,
    //     ) -> Result<(), DatabaseError> {
    //         let followed_user =
    //             users::find_by_username(&self.0, &to_be_followed.username).map_err(to_db_error)?;
    //         Ok(followers::follow(&self.0, follower.id, followed_user.id).map_err(to_db_error)?)
    //     }

    //     fn unfollow(
    //         &self,
    //         follower: &domain::User,
    //         to_be_unfollowed: &domain::Profile,
    //     ) -> Result<(), DatabaseError> {
    //         let unfollowed_user =
    //             users::find_by_username(&self.0, &to_be_unfollowed.username).map_err(to_db_error)?;
    //         Ok(
    //             followers::unfollow(&self.0, follower.id, unfollowed_user.id)
    //                 .map_err(OpaqueError::from)?,
    //         )
    //     }

    //     fn get_tags(&self) -> Result<HashSet<String>, DatabaseError> {
    //         Ok(articles::tags(&self.0).map_err(OpaqueError::from)?)
    //     }
}
