package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type BackgroundTask struct {
	api *openapi.APIClient
}

type BackgroundTaskListOptions struct {
	Iterator *string
	Limit    *int32
	Order    *Ordering
	Status   *BackgroundTaskStatus
	Task     *BackgroundTaskType
}

func (a *BackgroundTask) List(
	ctx context.Context,
	options *BackgroundTaskListOptions,
) (*ListResponseBackgroundTaskOut, error) {
	req := a.api.BackgroundTasksAPI.ListBackgroundTasks(ctx)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.Order != nil {
			req = req.Order(*options.Order)
		}
		if options.Status != nil {
			req = req.Status(*options.Status)
		}
		if options.Task != nil {
			req = req.Task(*options.Task)
		}
	}
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (a *BackgroundTask) Get(
	ctx context.Context,
	taskId string,
) (*BackgroundTaskOut, error) {
	req := a.api.BackgroundTasksAPI.GetBackgroundTask(ctx, taskId)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}
